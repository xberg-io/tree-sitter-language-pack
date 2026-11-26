package Shop::Counter;

use v5.38;
use strict;
use warnings;
use parent -norequire, 'Shop::Base';
use constant MAX_COUNT => 10;
use constant { MIN_COUNT => 0, STEP => 1 };

our $VERSION = '1.0';
our @ISA = ('Shop::Base');
our ($instances, %registry);

sub new {
    my ($class, %args) = @_;
    my $self = { count => $args{start} // MIN_COUNT };
    $instances++;
    return bless $self, $class;
}

sub increment {
    my ($self, $step) = @_;
    $self->{count} = clamp($self->{count} + ($step // STEP));
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

package Shop::Tally {
    use feature 'class';
    no warnings 'experimental::class';

    class Shop::Tally::Entry 1.0 :isa(Shop::Counter) {
        field $label :param = 'entry';
        field @marks;

        method describe {
            return sprintf '%s: %d', $label, scalar @marks;
        }
    }
}

1;
