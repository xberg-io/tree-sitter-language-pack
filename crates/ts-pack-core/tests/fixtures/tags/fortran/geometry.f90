module geometry
  implicit none
  real, parameter :: pi = 3.14159
  integer :: shape_count = 0

  type :: point
    real :: x, y
  contains
    procedure :: norm => point_norm
  end type point

  type, extends(point) :: labeled_point
    character(len=16) :: label
  end type labeled_point

  interface
    module function circle_area(r) result(a)
      real, intent(in) :: r
      real :: a
    end function circle_area
  end interface

  abstract interface
    function transform(v) result(w)
      real, intent(in) :: v
      real :: w
    end function transform
  end interface

  interface shape_area
    module procedure circle_area
  end interface shape_area

contains
  function point_norm(self) result(n)
    class(point), intent(in) :: self
    real :: n
    real :: sq
    sq = self%x**2 + self%y**2
    n = sqrt(sq)
  end function point_norm

  subroutine show(r)
    real, intent(in) :: r
    print *, circle_area(r)
  end subroutine show
end module geometry

submodule (geometry) geometry_impl
contains
  module procedure circle_area
    a = pi * r * r
  end procedure circle_area
end submodule geometry_impl

program main
  use geometry
  type(point) :: p
  p = point(1.0, 2.0)
  call show(2.0)
end program main
