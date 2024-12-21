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

    let isReportOrdered (report: int List) =
        let rec checkOrder list errCount =
            match list with
            | []
            | [ _ ] -> true
            | x :: y :: rest when x < y -> checkOrder (y :: rest) errCount
            | x :: y :: rest ->
                match errCount with
                | 1 -> false
                | _ -> checkOrder (x :: rest) (errCount + 1) || checkOrder (y :: rest) (errCount + 1)

        checkOrder report 0 || checkOrder (List.rev report) 0

    let isSafeReport (report: int List) =
        let rec checkSafety list errCount =
            match list with
            | []
            | [ _ ] -> true
            | x :: y :: rest when y - x >= 1 && y - x <= 3 -> checkSafety (y :: rest) errCount
            | x :: y :: rest ->
                match errCount with
                | 1 -> false
                | _ -> checkSafety (x :: rest) (errCount + 1) || checkSafety (y :: rest) (errCount + 1)
        
        checkSafety report 0 || checkSafety (List.rev report) 0

    System.IO.File.ReadLines("./Inputs/Day2.txt")
    |> Seq.map _.Split(" ", StringSplitOptions.RemoveEmptyEntries)
    |> Seq.map (Seq.map int)
    |> Seq.map Seq.toList
    |> Seq.filter isReportOrdered
    |> Seq.filter isSafeReport
    |> fun s -> Seq.iter (printfn "Valid: %A") s; s
    |> Seq.length
    |> printfn "Day 2, Solution 2: %i"
