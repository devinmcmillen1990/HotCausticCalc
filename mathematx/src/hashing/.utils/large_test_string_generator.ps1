# Example: Generate a 50MB file filled with 'A'
#   .\GenerateLargeString.ps1 -OutputFile "large_input.txt" -SizeInMB 50 -Character "A"
param(
    [string]    $OutputFile = "large_test_string_for_hashing.txt",
    [int]       $SizeInMB = 10,
    [string]    $Character = "A"
)

$characterCount = $SizeInMB * 1024 * 1024

Write-Host "Generating a string of $SizeInMB MB using character '$Character'..."
$largeString = ($Character * $characterCount)

Write-Host "Writing to $OutputFile..."
$largeString | Set-Content -Path $OutputFile

Write-Host "File generation complete: $OutputFile"