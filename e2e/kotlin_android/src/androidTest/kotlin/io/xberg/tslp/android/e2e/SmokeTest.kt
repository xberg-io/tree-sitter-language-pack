package io.xberg.tslp.android.e2e

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class SmokeTest {

    companion object {
        @BeforeClass
        @JvmStatic
        fun loadNativeLibrary() {
            System.loadLibrary("tree_sitter_language_pack_jni")
        }
    }

    @Test
    fun test_smoke_abl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_abl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_actionscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_actionscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ada() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ada */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_agda() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_agda */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_al() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_al */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_angular() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_angular */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_apex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_apex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_arduino() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_arduino */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_asciidoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_asciidoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_asm() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_asm */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_astro() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_astro */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_awk() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_awk */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bash() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bash */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bass() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bass */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_batch() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_batch */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_beancount() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_beancount */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bibtex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bibtex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bicep() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bicep */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bitbake() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bitbake */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_blade() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_blade */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_brightscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_brightscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_bsl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_bsl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_c() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_c */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_c3() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_c3 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_caddy() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_caddy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cairo() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cairo */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_capnp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_capnp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cedar() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cedar */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cedarschema() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cedarschema */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cel() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cel */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cfml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cfml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_chatito() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_chatito */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_chuck() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_chuck */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_circom() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_circom */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_clarity() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_clarity */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_clojure() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_clojure */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cmake() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cmake */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cobol() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cobol */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_comment() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_comment */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_commonlisp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_commonlisp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cooklang() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cooklang */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_corn() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_corn */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cpon() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cpon */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cpp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cpp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_crystal() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_crystal */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_csharp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_csharp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_css() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_css */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cst() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cst */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_csv() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_csv */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cuda() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cuda */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cue() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cue */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_cylc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_cylc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_d() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_d */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_dart() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_dart */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_desktop() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_desktop */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_devicetree() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_devicetree */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_dhall() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_dhall */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_diff() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_diff */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_djot() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_djot */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_dockerfile() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_dockerfile */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_dot() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_dot */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_doxygen() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_doxygen */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_dtd() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_dtd */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_earthfile() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_earthfile */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ebnf() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ebnf */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_editorconfig() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_editorconfig */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_eds() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_eds */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_eex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_eex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_elisp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_elisp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_elixir() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_elixir */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_elm() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_elm */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_elsa() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_elsa */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_elvish() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_elvish */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_embeddedtemplate() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_embeddedtemplate */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_enforce() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_enforce */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_erlang() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_erlang */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_facility() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_facility */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_faust() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_faust */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fennel() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fennel */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fidl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fidl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_firrtl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_firrtl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fish() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fish */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_foam() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_foam */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_forth() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_forth */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fortran() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fortran */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fsharp() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fsharp */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_fsharp_signature() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_fsharp_signature */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_func() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_func */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gap() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gap */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gdscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gdscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gdshader() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gdshader */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_git_config() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_git_config */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_git_rebase() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_git_rebase */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gitattributes() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gitattributes */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gitcommit() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gitcommit */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gitignore() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gitignore */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gleam() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gleam */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_glimmer() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_glimmer */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_glsl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_glsl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gn() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gn */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gnuplot() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gnuplot */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_go() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_go */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_godot_resource() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_godot_resource */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gomod() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gomod */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gosum() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gosum */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gotmpl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gotmpl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gowork() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gowork */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gpg() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gpg */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_graphql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_graphql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gren() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gren */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_groovy() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_groovy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_gstlaunch() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_gstlaunch */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hack() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hack */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hare() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hare */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_haskell() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_haskell */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_haxe() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_haxe */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hcl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hcl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_heex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_heex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hjson() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hjson */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hlsl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hlsl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hocon() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hocon */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hoon() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hoon */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_html() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_html */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_htmldjango() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_htmldjango */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_http() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_http */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hurl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hurl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_hyprlang() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_hyprlang */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_idris() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_idris */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ini() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ini */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ispc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ispc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_jai() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_jai */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_janet() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_janet */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_java() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_java */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_javadoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_javadoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_javascript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_javascript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_jinja2() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_jinja2 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_jq() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_jq */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_jsdoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_jsdoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_json() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_json */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_json5() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_json5 */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_jsonnet() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_jsonnet */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_julia() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_julia */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_just() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_just */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_kcl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_kcl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_kconfig() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_kconfig */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_kdl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_kdl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_kotlin() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_kotlin */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_latex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_latex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_lean() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_lean */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ledger() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ledger */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_less() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_less */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_linkerscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_linkerscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_liquid() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_liquid */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_llvm() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_llvm */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_lua() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_lua */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_luadoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_luadoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_luap() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_luap */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_luau() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_luau */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_magik() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_magik */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_make() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_make */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_markdown() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_markdown */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_markdown_inline() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_markdown_inline */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_matlab() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_matlab */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_mermaid() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_mermaid */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_meson() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_meson */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_mlir() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_mlir */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_mojo() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_mojo */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_move() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_move */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nasm() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nasm */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_netlinx() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_netlinx */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nginx() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nginx */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nickel() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nickel */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nim() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nim */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ninja() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ninja */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nix() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nix */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_norg() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_norg */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_norg_meta() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_norg_meta */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nqc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nqc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_nushell() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_nushell */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_objc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_objc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ocaml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ocaml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ocaml_interface() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ocaml_interface */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ocamllex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ocamllex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_odin() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_odin */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_openscad() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_openscad */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_org() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_org */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pascal() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pascal */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pem() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pem */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_perl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_perl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pgn() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pgn */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_php() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_php */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_phpdoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_phpdoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pkl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pkl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_po() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_po */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_poe_filter() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_poe_filter */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pony() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pony */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_postscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_postscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_powershell() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_powershell */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_printf() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_printf */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_prisma() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_prisma */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_prolog() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_prolog */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_promql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_promql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_properties() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_properties */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_proto() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_proto */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_prql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_prql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_psv() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_psv */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pug() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pug */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_puppet() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_puppet */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_purescript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_purescript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_pymanifest() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_pymanifest */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_python() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_python */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_qmldir() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_qmldir */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_qmljs() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_qmljs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_query() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_query */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_r() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_r */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_racket() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_racket */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rasi() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rasi */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_razor() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_razor */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rbs() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rbs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_re2c() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_re2c */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_readline() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_readline */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_regex() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_regex */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rego() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rego */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_requirements() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_requirements */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rescript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rescript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_robot() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_robot */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_roc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_roc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ron() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ron */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rst() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rst */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rtf() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rtf */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ruby() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ruby */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_rust() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_rust */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_scala() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_scala */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_scheme() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_scheme */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_scss() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_scss */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_slang() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_slang */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_smali() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_smali */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_smalltalk() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_smalltalk */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_smithy() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_smithy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_snakemake() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_snakemake */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_solidity() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_solidity */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_souffle() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_souffle */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sourcepawn() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sourcepawn */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sparql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sparql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sql() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sql */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sql_bigquery() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sql_bigquery */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_squirrel() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_squirrel */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ssh_config() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ssh_config */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_stan() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_stan */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_starlark() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_starlark */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_superhtml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_superhtml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_svelte() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_svelte */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_sway() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_sway */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_swift() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_swift */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_systemverilog() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_systemverilog */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tablegen() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tablegen */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tact() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tact */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tcl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tcl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_teal() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_teal */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_templ() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_templ */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tera() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tera */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_terraform() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_terraform */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_test() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_test */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_textproto() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_textproto */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_thrift() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_thrift */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tlaplus() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tlaplus */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tmux() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tmux */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_todotxt() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_todotxt */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_toml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_toml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tsv() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tsv */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_tsx() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_tsx */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_turtle() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_turtle */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_twig() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_twig */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_typescript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_typescript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_typespec() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_typespec */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_typoscript() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_typoscript */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_typst() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_typst */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_udev() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_udev */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ungrammar() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ungrammar */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_uxntal() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_uxntal */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_v() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_v */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vb() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vb */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_verilog() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_verilog */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vhdl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vhdl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vhs() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vhs */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vim() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vim */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vimdoc() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vimdoc */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vrl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vrl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_vue() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_vue */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_wast() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_wast */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_wat() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_wat */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_wgsl() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_wgsl */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_wgsl_bevy() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_wgsl_bevy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_wit() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_wit */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_x86asm() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_x86asm */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_xcompose() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_xcompose */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_xml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_xml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_yaml() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_yaml */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_yuck() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_yuck */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_zig() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_zig */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ziggy() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ziggy */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_ziggy_schema() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_ziggy_schema */)
        // TODO: assert result is not an error
    }

    @Test
    fun test_smoke_zsh() {
        val client = TreeSitterLanguagePack()
        val result = client.process(/* fixture: smoke_zsh */)
        // TODO: assert result is not an error
    }

}
