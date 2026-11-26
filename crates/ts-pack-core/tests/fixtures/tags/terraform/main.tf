terraform {
  required_version = ">= 1.5"
}

provider "aws" {
  region = "eu-west-1"
}

resource "aws_s3_bucket" "logs" {
  bucket = "logs"
}

data "aws_ami" "ubuntu" {
  most_recent = true
}

variable "env" {
  type = string
}

output "bucket_arn" {
  value = aws_s3_bucket.logs.arn
}

module "vpc" {
  source = "./vpc"
}

locals {
  name_prefix = "app"
  tags        = {}
}
