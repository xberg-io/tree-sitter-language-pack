class greeter (
  String $version = '1.2.0',
  Integer $port   = 8080,
) inherits greeter::params {
  include greeter::install

  $config_dir = '/etc/greeter'

  file { "${config_dir}/greeter.conf":
    ensure  => file,
    content => template('greeter/greeter.conf.erb'),
    notify  => Service['greeter'],
  }

  service { 'greeter':
    ensure => running,
  }
}

define greeter::site (String $hostname) {
  file { "/etc/greeter/sites/${title}.conf":
    content => $hostname,
  }
}

function greeter::url(String $host) >> String {
  "http://${host}/"
}

node 'web01.example.com' {
  include greeter
  greeter::site { 'main': hostname => 'example.com' }
}
