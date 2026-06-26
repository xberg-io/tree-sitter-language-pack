package io.xberg.tslp.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RegistryTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("tree_sitter_language_pack_jni")
        }
    }

    @Test
    fun test_get_language_python() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: get_language_python */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_language_unknown() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: get_language_unknown */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_parser_python() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: get_parser_python */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_get_parser_unknown() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: get_parser_unknown */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_get_language_alias() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_get_language_alias */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_get_parser_alias() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_get_parser_alias */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_has_language_alias() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_has_language_alias */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_has_language_false() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_has_language_false */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_has_language_true() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_has_language_true */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_language_count() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_language_count */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_registry_list_languages() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: registry_list_languages */)
        // TODO: assert result is not an error
    }

}
