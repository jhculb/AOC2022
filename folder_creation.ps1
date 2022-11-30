$counter = 1
while($counter -lt 32){
    $main_folder = '.\Day {0}' -f $counter
    $part1 = Join-Path -Path $main_folder -ChildPath "Part 1"
    $part2 = Join-Path -Path $main_folder -ChildPath "Part 2"
    New-Item -Path $part1 -ItemType Directory
    New-Item -Path $part2 -ItemType Directory
    $counter += 1;
}