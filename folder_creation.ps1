$counter = 1
$parts = (1, 2)
while($counter -lt 32){
    $main_folder = '.\Day {0}' -f $counter
    $data_txt = Join-Path $main_folder -ChildPath "data.txt"
    $input_txt = Join-Path $main_folder -ChildPath "input1.txt"
    $input2_txt = Join-Path $main_folder -ChildPath "input2.txt"
    New-Item -Path $data_txt
    New-Item -Path $input_txt
    New-Item -Path $input2_txt
    ForEach ($part in $parts){
        $endbit = 'Part {0}' -f $part
        $part1 = Join-Path -Path $main_folder -ChildPath $endbit
        New-Item -Path $part1 -ItemType Directory
    }
    $counter += 1;
}