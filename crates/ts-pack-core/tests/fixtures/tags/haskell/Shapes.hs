module Shapes (Shape (..), area) where

import Data.List (sortOn)

data Shape = Circle Double | Square Double
  deriving (Show)

newtype Name = Name String

data Point = Point { px :: Double, py :: Double }

type Area = Double

class HasArea a where
  areaOf :: a -> Area

instance HasArea Shape where
  areaOf = area

area :: Shape -> Area
area (Circle r) = pi * r * r
area (Square s) = s * s

origin :: Point
origin = Point 0 0

largest :: [Shape] -> [Shape]
largest shapes = reverse (sortOn area shapes)
  where
    helper x = x
