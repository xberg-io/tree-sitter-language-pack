xquery version "3.1";

module namespace geo = "http://example.com/geo";

declare variable $geo:pi := 3.14;

declare function geo:area($r as xs:double) as xs:double {
  $geo:pi * $r * $r
};

declare function geo:double($x) {
  let $y := $x * 2
  return geo:area($y)
};
