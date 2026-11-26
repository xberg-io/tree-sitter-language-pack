module Shapes where

import Prelude

foreign import log :: String -> Number

data Shape = Circle Number | Square Number

newtype Name = Name String

type Area = Number

class HasArea a where
  areaOf :: a -> Area

instance hasAreaShape :: HasArea Shape where
  areaOf = area

area :: Shape -> Area
area (Circle r) = 3.14 * r * r
area (Square s) = s * s

double :: Int -> Int
double x = helper x
  where
  helper y = y * 2
