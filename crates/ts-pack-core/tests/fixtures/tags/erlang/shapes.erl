-module(shapes).
-export([area/1, perimeter/1]).

-define(PI, 3.14159).

-record(circle, {radius = 1.0 :: float()}).

-type shape() :: {square, number()} | #circle{}.

-opaque handle() :: reference().

-callback handle_event(term()) -> ok.

-spec area(shape()) -> number().
area({square, Side}) ->
    Side * Side;
area(#circle{radius = R}) ->
    ?PI * R * R.

perimeter({square, Side}) ->
    4 * Side;
perimeter(#circle{radius = R}) ->
    2 * ?PI * R.

helper(X) ->
    lists:map(fun(Y) -> Y + X end, [1, 2, 3]).
