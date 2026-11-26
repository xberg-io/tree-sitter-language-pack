SUMMARY = "Hello world example"
LICENSE = "MIT"
SRC_URI = "file://hello.c"

inherit pkgconfig update-rc.d
require hello-common.inc

DEPENDS:append = " zlib"
S = "${WORKDIR}"

def hello_flags(d):
    return d.getVar("CFLAGS") or ""

do_compile() {
    ${CC} ${LDFLAGS} hello.c -o hello
}

python do_report() {
    bb.note("built %s" % d.getVar("PN"))
}

addtask report after do_compile before do_install
