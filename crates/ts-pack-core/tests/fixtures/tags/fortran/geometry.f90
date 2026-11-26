module geometry
  implicit none
contains
  function area(r) result(a)
    real, intent(in) :: r
    real :: a
    a = 3.14 * r * r
  end function area

  subroutine show(r)
    real, intent(in) :: r
    print *, area(r)
  end subroutine show
end module geometry

program main
  use geometry
  call show(2.0)
end program main
