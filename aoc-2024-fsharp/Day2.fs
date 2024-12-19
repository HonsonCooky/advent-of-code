module aoc_2024_fsharp.Day2

open System
let d2s1 () =
    System.IO.File.ReadLines("./Examples/Day2.txt")
    |> Seq.map _.Split(" ", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map (Seq.map int)
    |> Seq.map (fun s -> Seq.take 2 s |> Seq.toList |> (fun s' -> if s'[0] < s'[1] then s else Seq.rev s))
    |> Seq.map Seq.pairwise
    |> Seq.filter (Seq.forall (fun (a, b) -> a - b >= 1 && a - b <= 3))
    |> Seq.length
    |> printfn("Day 2, Solution 1: %i")
