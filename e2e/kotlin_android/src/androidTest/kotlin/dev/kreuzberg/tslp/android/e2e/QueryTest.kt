package dev.kreuzberg.tslp.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class QueryTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("tree_sitter_language_pack_jni")
        }
    }

    @Test
    fun test_highlights_nonexistent_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: highlights_nonexistent_language */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_highlights_query_rust() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: highlights_query_rust */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_highlights_query_unknown_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: highlights_query_unknown_language */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_injections_query_rust() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: injections_query_rust */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_injections_query_unknown_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: injections_query_unknown_language */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_locals_query_cue() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: locals_query_cue */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_locals_query_unknown_language() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: locals_query_unknown_language */)
        // TODO: assert result is not an error
    }

}
