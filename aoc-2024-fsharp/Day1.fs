module aoc_2024_fsharp.Day1

open System

let d1s1 () =
    System.IO.File.ReadLines("./Inputs/Day1.txt")
    |> Seq.map (_.Split(" ", StringSplitOptions.RemoveEmptyEntries))
    |> Seq.map (Array.map int)
    |> Seq.map (fun arr -> (arr[0], arr[1]))
    |> Seq.fold (fun (ls, rs) (l, r) -> (l :: ls, r :: rs)) ([], [])
    |> fun (ls, rs) -> (ls |> List.sort, rs |> List.sort)
    |> fun (ls, rs) -> List.fold2 (fun acc l r -> abs (l - r) + acc) 0 ls rs
    |> printfn ("%i")

let d1s2 () =
    System.IO.File.ReadLines("./Inputs/Day1.txt")
    |> Seq.map (_.Split(" ", StringSplitOptions.RemoveEmptyEntries))
    |> Seq.map (Array.map int)
    |> Seq.map (fun arr -> (arr[0], arr[1]))
    |> Seq.fold (fun (ls, rs) (l, r) -> (l :: ls, r :: rs)) ([], [])
    |> fun (ls, rs) -> (ls |> List.map (fun left -> (left, rs |> List.filter ((=) left) |> List.length)))
    |> List.fold (fun acc (v, c) -> acc + (v * c)) 0
    |> printfn("%i")
