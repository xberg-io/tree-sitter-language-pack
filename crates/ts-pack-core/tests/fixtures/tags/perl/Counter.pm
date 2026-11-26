package Shop::Counter;

use strict;
use warnings;
use constant MAX_COUNT => 10;

our $VERSION = '1.0';

sub new {
    my ($class, %args) = @_;
    my $self = { count => $args{start} // 0 };
    return bless $self, $class;
}

sub increment {
    my ($self, $step) = @_;
    $self->{count} = clamp($self->{count} + $step);
    $self->report();
    return $self->{count};
}

sub report {
    my ($self) = @_;
    printf "count: %d\n", $self->{count};
}

sub clamp {
    my ($value) = @_;
    return $value > MAX_COUNT ? MAX_COUNT : $value;
}

1;
