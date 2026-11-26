set -gx PREFIX /usr/local
set TARGETS api worker
set -U -x EDITOR nvim

function build --argument-names name
    set -l out $PREFIX/bin/$name
    echo "building $name into $out"
end

function deploy --description 'Build and ship every target'
    for target in $TARGETS
        build $target
    end
end

deploy
