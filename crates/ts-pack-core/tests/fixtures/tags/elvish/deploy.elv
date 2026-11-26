use str
use ./lib/util

var prefix = /usr/local
var targets = [api worker]

fn build {|name|
  var out = $prefix/bin/$name
  echo 'building '$name' into '$out
}

fn deploy {|@names|
  for name $names {
    build $name
  }
}

set targets = [(str:split , api,worker)]
deploy $@targets
