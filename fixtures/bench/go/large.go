// Package tspack provides Go bindings for tree-sitter-language-pack via cgo.
//
// It wraps the C-FFI layer (ts-pack-ffi) to provide access to 165+ tree-sitter
// language grammars through a safe, idiomatic Go API.
//
// Language names are plain strings such as "python", "rust", "javascript", etc.
// Use [Registry.AvailableLanguages] to discover all supported names at runtime,
// or [Registry.HasLanguage] to check for a specific language before loading it.
//
// # Usage
//
//	reg, err := tspack.NewRegistry()
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer reg.Close()
//
//	langPtr, err := reg.GetLanguage("python")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	// langPtr is an unsafe.Pointer to a TSLanguage struct that can be
//	// passed to a tree-sitter Go wrapper such as go-tree-sitter.
//
// # Concurrency
//
// The [Registry] type is safe for concurrent use from multiple goroutines.
// All exported methods acquire the appropriate lock before accessing the
// underlying C registry.
package tspack

/*
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
import (
	"encoding/json"
	"errors"
	"fmt"
	"runtime"
	"sync"
	"unsafe"
)

// Registry wraps a TsPackRegistry handle and provides access to tree-sitter
// language grammars. It is safe for concurrent use from multiple goroutines.
//
// A Registry must be created via [NewRegistry] and should be closed with
// [Registry.Close] when no longer needed. If Close is not called, the
// finalizer will release the underlying C resources during garbage collection.
type Registry struct {
	mu  sync.RWMutex
	ptr *C.TsPackRegistry
}

// lastError retrieves the last error message from the FFI layer.
// Returns nil if no error is set.
//
// IMPORTANT: The caller must hold the OS thread locked (runtime.LockOSThread)
// because ts_pack_last_error uses thread-local storage. This function must be
// called on the same OS thread as the FFI call that produced the error.
func lastError() error {
	cerr := C.ts_pack_last_error()
	if cerr == nil {
		return nil
	}
	msg := C.GoString(cerr)
	return errors.New(msg)
}

// NewRegistry creates a new language registry containing all available
// tree-sitter grammars. The registry is automatically freed when garbage
// collected, but callers may also call Close for deterministic cleanup.
//
// Returns an error if the underlying FFI call fails.
func NewRegistry() (*Registry, error) {
	// Lock OS thread so the FFI call and subsequent error check use the same
	// thread-local storage.
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	ptr := C.ts_pack_registry_new()
	if ptr == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: failed to create registry: %w", err)
		}
		return nil, errors.New("tspack: failed to create registry: unknown error")
	}

	r := &Registry{ptr: ptr}
	runtime.SetFinalizer(r, (*Registry).free)
	return r, nil
}

// free releases the underlying C registry. Called by the finalizer.
func (r *Registry) free() {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.ptr != nil {
		C.ts_pack_registry_free(r.ptr)
		r.ptr = nil
	}
}

// Close explicitly frees the underlying C registry. After Close is called,
// all other methods will return errors or zero values.
//
// It is safe to call Close multiple times.
func (r *Registry) Close() {
	r.free()
	runtime.SetFinalizer(r, nil)
}

// ensureOpen returns an error if the registry has been closed.
func (r *Registry) ensureOpen() error {
	if r.ptr == nil {
		return errors.New("tspack: registry is closed")
	}
	return nil
}

// GetLanguage returns a pointer to the TSLanguage for the given language name.
//
// The returned unsafe.Pointer can be cast to the appropriate type by consumers
// (e.g., go-tree-sitter's Language type). The pointer remains valid for the
// lifetime of the Registry.
//
// Returns an error if the language is not found or the registry is closed.
func (r *Registry) GetLanguage(name string) (unsafe.Pointer, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return nil, err
	}

	cname := C.CString(name)
	defer C.free(unsafe.Pointer(cname))

	// Lock OS thread so the FFI call and error check share thread-local storage.
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	lang := C.ts_pack_get_language(r.ptr, cname)

	if lang == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: language %q: %w", name, err)
		}
		return nil, fmt.Errorf("tspack: language %q not found", name)
	}

	return unsafe.Pointer(lang), nil
}

// LanguageCount returns the number of available languages in the registry.
// Returns 0 if the registry is closed.
func (r *Registry) LanguageCount() int {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return 0
	}

	return int(C.ts_pack_language_count(r.ptr))
}

// LanguageNameAt returns the language name at the given index. Valid indices
// are in the range [0, LanguageCount()). Returns an error if the index is out
// of bounds or the registry is closed.
func (r *Registry) LanguageNameAt(index int) (string, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return "", err
	}

	cname := C.ts_pack_language_name_at(r.ptr, C.uintptr_t(index))

	if cname == nil {
		return "", fmt.Errorf("tspack: index %d out of bounds", index)
	}

	name := C.GoString(cname)
	C.ts_pack_free_string((*C.char)(unsafe.Pointer(cname)))

	return name, nil
}

// HasLanguage reports whether the registry contains a grammar for the named
// language. Returns false if the registry is closed.
func (r *Registry) HasLanguage(name string) bool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return false
	}

	cname := C.CString(name)
	defer C.free(unsafe.Pointer(cname))

	return bool(C.ts_pack_has_language(r.ptr, cname))
}

// DetectLanguage detects a language name from a file path or extension.
// Returns an empty string if the extension is not recognized.
func DetectLanguage(path string) string {
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_detect_language(cpath)
	if result == nil {
		return ""
	}
	defer C.ts_pack_free_string(result)
	return C.GoString(result)
}

// DetectLanguageFromContent detects a language name from file content using
// shebang-based detection. Returns an empty string if no shebang is recognized.
func DetectLanguageFromContent(content string) string {
	ccontent := C.CString(content)
	defer C.free(unsafe.Pointer(ccontent))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_detect_language_from_content(ccontent)
	if result == nil {
		return ""
	}
	defer C.ts_pack_free_string(result)
	return C.GoString(result)
}

// GetHighlightsQuery returns the bundled highlights query for the given language.
// Returns an empty string if no bundled query is available.
func GetHighlightsQuery(language string) string {
	clang := C.CString(language)
	defer C.free(unsafe.Pointer(clang))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_get_highlights_query(clang)
	if result == nil {
		return ""
	}
	defer C.ts_pack_free_string(result)
	return C.GoString(result)
}

// GetInjectionsQuery returns the bundled injections query for the given language.
// Returns an empty string if no bundled query is available.
func GetInjectionsQuery(language string) string {
	clang := C.CString(language)
	defer C.free(unsafe.Pointer(clang))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_get_injections_query(clang)
	if result == nil {
		return ""
	}
	defer C.ts_pack_free_string(result)
	return C.GoString(result)
}

// GetLocalsQuery returns the bundled locals query for the given language.
// Returns an empty string if no bundled query is available.
func GetLocalsQuery(language string) string {
	clang := C.CString(language)
	defer C.free(unsafe.Pointer(clang))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_get_locals_query(clang)
	if result == nil {
		return ""
	}
	defer C.ts_pack_free_string(result)
	return C.GoString(result)
}

// AvailableLanguages returns a slice of all language names in the registry.
// Returns nil if the registry is closed.
func (r *Registry) AvailableLanguages() []string {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return nil
	}

	count := int(C.ts_pack_language_count(r.ptr))
	if count == 0 {
		return nil
	}

	languages := make([]string, 0, count)
	for i := 0; i < count; i++ {
		cname := C.ts_pack_language_name_at(r.ptr, C.uintptr_t(i))
		if cname == nil {
			continue
		}
		languages = append(languages, C.GoString(cname))
		C.ts_pack_free_string((*C.char)(unsafe.Pointer(cname)))
	}

	return languages
}

// Process extracts file intelligence (and optionally chunks) from the given
// source code using a ProcessConfig. The config specifies the language and
// which extraction features to enable.
//
// When config.ChunkMaxSize is non-nil, chunking is also performed.
//
// Returns a typed ProcessResult with deserialized metadata and chunks.
func (r *Registry) Process(source string, config ProcessConfig) (*ProcessResult, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return nil, err
	}

	configJSON, err := json.Marshal(config)
	if err != nil {
		return nil, fmt.Errorf("tspack: failed to serialize config: %w", err)
	}

	csource := C.CString(source)
	defer C.free(unsafe.Pointer(csource))

	cconfigJSON := C.CString(string(configJSON))
	defer C.free(unsafe.Pointer(cconfigJSON))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_process(r.ptr, csource, C.uintptr_t(len(source)), cconfigJSON)

	if result == nil {
		if ffiErr := lastError(); ffiErr != nil {
			return nil, fmt.Errorf("tspack: process %q: %w", config.Language, ffiErr)
		}
		return nil, fmt.Errorf("tspack: process %q failed", config.Language)
	}
	defer C.ts_pack_free_string(result)

	rawJSON := C.GoString(result)

	var processResult ProcessResult
	if err := json.Unmarshal([]byte(rawJSON), &processResult); err != nil {
		return nil, fmt.Errorf("tspack: failed to deserialize process result: %w", err)
	}
	return &processResult, nil
}

// Init initializes the language pack with configuration.
// configJSON is a JSON string with optional fields:
//   - "cache_dir" (string): override default cache directory
//   - "languages" (array): languages to pre-download
//   - "groups" (array): language groups to pre-download
//
// Returns an error if initialization fails.
func Init(configJSON string) error {
	cconfigJSON := C.CString(configJSON)
	defer C.free(unsafe.Pointer(cconfigJSON))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	rc := C.ts_pack_init(cconfigJSON)
	if rc != 0 {
		if err := lastError(); err != nil {
			return fmt.Errorf("tspack: init failed: %w", err)
		}
		return errors.New("tspack: init failed")
	}

	return nil
}

// Configure configures the language pack cache directory without downloading.
// configJSON is a JSON string with optional fields:
//   - "cache_dir" (string): override default cache directory
//
// Returns an error if configuration fails.
func Configure(configJSON string) error {
	cconfigJSON := C.CString(configJSON)
	defer C.free(unsafe.Pointer(cconfigJSON))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	rc := C.ts_pack_configure(cconfigJSON)
	if rc != 0 {
		if err := lastError(); err != nil {
			return fmt.Errorf("tspack: configure failed: %w", err)
		}
		return errors.New("tspack: configure failed")
	}

	return nil
}

// Download downloads specific languages to the cache.
// Returns the number of newly downloaded languages.
func Download(languages []string) (int, error) {
	if len(languages) == 0 {
		return 0, nil
	}

	cnames := make([]*C.char, len(languages))
	for i, name := range languages {
		cnames[i] = C.CString(name)
		defer C.free(unsafe.Pointer(cnames[i]))
	}

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	rc := C.ts_pack_download((**C.char)(unsafe.Pointer(&cnames[0])), C.uintptr_t(len(languages)))

	if rc < 0 {
		if err := lastError(); err != nil {
			return 0, fmt.Errorf("tspack: download failed: %w", err)
		}
		return 0, errors.New("tspack: download failed")
	}

	return int(rc), nil
}

// DownloadAll downloads all available languages from the remote manifest.
// Returns the number of newly downloaded languages.
func DownloadAll() (int, error) {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	rc := C.ts_pack_download_all()

	if rc < 0 {
		if err := lastError(); err != nil {
			return 0, fmt.Errorf("tspack: download all failed: %w", err)
		}
		return 0, errors.New("tspack: download all failed")
	}

	return int(rc), nil
}

// ManifestLanguages returns all language names available in the remote manifest.
func ManifestLanguages() ([]string, error) {
	var count C.uintptr_t

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	arr := C.ts_pack_manifest_languages(&count)
	if arr == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: manifest languages failed: %w", err)
		}
		return nil, errors.New("tspack: manifest languages failed")
	}
	defer C.ts_pack_free_string_array(arr)

	languages := make([]string, int(count))
	for i := 0; i < int(count); i++ {
		// Unsafe pointer arithmetic to get each string pointer
		ptr := *(**C.char)(unsafe.Pointer(uintptr(unsafe.Pointer(arr)) + uintptr(i)*unsafe.Sizeof((*C.char)(nil))))
		languages[i] = C.GoString(ptr)
		// Free each individual string before freeing the array
		C.ts_pack_free_string(ptr)
	}

	return languages, nil
}

// DownloadedLanguages returns all languages that are already downloaded and cached locally.
func DownloadedLanguages() ([]string, error) {
	var count C.uintptr_t

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	arr := C.ts_pack_downloaded_languages(&count)
	if arr == nil {
		return nil, errors.New("tspack: failed to get downloaded languages")
	}
	defer C.ts_pack_free_string_array(arr)

	languages := make([]string, int(count))
	for i := 0; i < int(count); i++ {
		ptr := *(**C.char)(unsafe.Pointer(uintptr(unsafe.Pointer(arr)) + uintptr(i)*unsafe.Sizeof((*C.char)(nil))))
		languages[i] = C.GoString(ptr)
		// Free each individual string before freeing the array
		C.ts_pack_free_string(ptr)
	}

	return languages, nil
}

// CleanCache deletes all cached parser shared libraries.
func CleanCache() error {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	rc := C.ts_pack_clean_cache()
	if rc != 0 {
		if err := lastError(); err != nil {
			return fmt.Errorf("tspack: clean cache failed: %w", err)
		}
		return errors.New("tspack: clean cache failed")
	}

	return nil
}

// CacheDir returns the effective cache directory path.
func CacheDir() (string, error) {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	cstr := C.ts_pack_cache_dir()
	if cstr == nil {
		if err := lastError(); err != nil {
			return "", fmt.Errorf("tspack: cache dir failed: %w", err)
		}
		return "", errors.New("tspack: cache dir failed")
	}
	defer C.ts_pack_free_string(cstr)

	return C.GoString(cstr), nil
}
