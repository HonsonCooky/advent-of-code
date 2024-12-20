module aoc_2024_fsharp.Day2

open System

let d2s1 () =
    System.IO.File.ReadLines("./Inputs/Day2.txt")
    |> Seq.map _.Split(" ", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map (Seq.map int)
    |> Seq.map (fun s -> Seq.take 2 s |> Seq.toList |> (fun s' -> if s'[0] < s'[1] then s else Seq.rev s))
    |> Seq.map Seq.pairwise
    |> Seq.filter (Seq.forall (fun (a, b) -> b - a >= 1 && b - a <= 3))
    |> Seq.length
    |> printfn ("Day 2, Solution 1: %i")

let d2s2 () =

    let isSafeReport (report: int List) =
        true

    System.IO.File.ReadLines("./Inputs/Day2.txt")
    |> Seq.map _.Split(" ", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map (Seq.map int)
    |> Seq.map Seq.toList
    |> Seq.filter isSafeReport
    |> Seq.length
    |> printfn "Day 2, Solution 2: %i"
