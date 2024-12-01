import gleam/list
import simplifile
import gleam/io
import gleam/string
import gleam/int

pub fn main() {
  let filepath = "./input.txt"
  let assert Ok(contents) = simplifile.read(from: filepath)
  let #(left, right) = string.split(contents, on: "\n")
    |> list.filter(fn(line) { !string.is_empty(line) })
    |> list.map(fn(line) {
      let assert Ok(#(left, right)) = string.split_once(line, "   ")
      let assert Ok(left) = int.parse(left)
      let assert Ok(right) = int.parse(right)
      #(left, right)
    })
    |> list.unzip
  let left = list.sort(left, int.compare)
  let right = list.sort(right, int.compare)
  let assert Ok(difference) = list.zip(left, right)
    |> list.map(fn(list) {
      let #(left, right) = list
      int.subtract(left, right)
        |> int.absolute_value
    })
    |> list.reduce(fn(acc, cur) { acc + cur })
  io.println(string.concat(["Part one: ", int.to_string(difference)]))

  let assert Ok(similarity) = list.map(left, fn(item) {
    let times_in_right_list = list.count(right, fn(v) { v == item })
    item * times_in_right_list
  })
    |> list.reduce(fn(acc, cur) { acc + cur })
  io.println(string.concat(["Part two: ", int.to_string(similarity)]))
}
