# Test script for path parameters
Write-Host "Testing path parameters..." -ForegroundColor Cyan

Write-Host "`nTest 1: GET /users/123" -ForegroundColor Yellow
$response1 = Invoke-WebRequest -Uri "http://localhost:8080/users/123" -UseBasicParsing
Write-Host $response1.Content -ForegroundColor Green

Write-Host "`nTest 2: GET /users/456" -ForegroundColor Yellow
$response2 = Invoke-WebRequest -Uri "http://localhost:8080/users/456" -UseBasicParsing
Write-Host $response2.Content -ForegroundColor Green

Write-Host "`nTest 3: GET /posts/2024/01/hello-world" -ForegroundColor Yellow
$response3 = Invoke-WebRequest -Uri "http://localhost:8080/posts/2024/01/hello-world" -UseBasicParsing
Write-Host $response3.Content -ForegroundColor Green

Write-Host "`nTest 4: GET /api/test (no params)" -ForegroundColor Yellow
$response4 = Invoke-WebRequest -Uri "http://localhost:8080/api/test" -UseBasicParsing
Write-Host $response4.Content -ForegroundColor Green

Write-Host "`nAll tests completed!" -ForegroundColor Cyan
