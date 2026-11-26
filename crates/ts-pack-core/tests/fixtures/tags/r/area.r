library(methods)
require(R6)

MAX_RADIUS <- 100
default_unit = "cm"

square <- function(x) {
  local_value <- x^2
  local_value
}

area = function(r) pi * square(r)

"circumference" <- function(r) 2 * pi * r

(function(r) 2 * r) -> diameter

scale <<- \(x, factor) x * factor

setClass("Shape", representation(name = "character"))

Circle <- setClass("Circle", contains = "Shape", representation(radius = "numeric"))

setGeneric("describe", function(shape) standardGeneric("describe"))

setMethod("describe", "Circle", function(shape) cat(shape@name, area(shape@radius)))

Canvas <- R6Class("Canvas",
  public = list(
    shapes = NULL,
    add = function(shape) {
      self$shapes <- c(self$shapes, shape)
      invisible(self)
    }
  )
)

print(area(2))
stats::median(c(1, 2, 3))
