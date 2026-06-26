package io.xberg.tslp.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DownloadTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("tree_sitter_language_pack_jni")
        }
    }

    @Test
    fun test_download_cache_dir() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_cache_dir */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_clean_cache() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_clean_cache */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_configure_custom_dir() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_configure_custom_dir */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_downloaded_empty() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_downloaded_empty */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_empty_list() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_empty_list */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_init_default() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_init_default */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_invalid_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_invalid_language */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_manifest_languages() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_manifest_languages */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_multiple_languages() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_multiple_languages */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_download_single_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: download_single_language */)
        // TODO: assert result is not an error
    }

}
