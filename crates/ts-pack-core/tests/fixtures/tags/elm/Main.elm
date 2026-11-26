module Main exposing (main)


type Color
    = Red
    | Green


type alias Point =
    { x : Int, y : Int }


add : Int -> Int -> Int
add a b =
    a + b


main =
    add 1 2
