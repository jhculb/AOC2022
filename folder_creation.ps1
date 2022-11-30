$counter = 1
$parts = (1, 2)
while($counter -lt 32){
    $main_folder = '.\Day {0}' -f $counter
    ForEach ($part in $parts){
        $endbit = 'Part {0}' -f $part
        $part1 = Join-Path -Path $main_folder -ChildPath $endbit
        New-Item -Path $part1 -ItemType Directory
    }
    $counter += 1;
}