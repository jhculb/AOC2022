$counter = 1
while($counter -lt 32){
    $main_folder = 'day_{0}' -f $counter
    $main_folder_create_command = {cargo new $main_folder}
    Invoke-Command -ScriptBlock $main_folder_create_command
    $resource_folder = $main_folder + "\src\resource"
    New-Item -Path $resource_folder -ItemType Directory
    $data_txt = Join-Path $resource_folder -ChildPath "data.txt"
    $input_txt = Join-Path $resource_folder -ChildPath "input1.txt"
    $input2_txt = Join-Path $resource_folder -ChildPath "input2.txt"
    New-Item -Path $data_txt
    New-Item -Path $input_txt
    New-Item -Path $input2_txt
    $counter += 1;
}