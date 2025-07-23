 # Get vehicle information
Invoke-RestMethod -Uri "http://localhost:5500/vehicle" -Method Get

# Post vehicle information
Invoke-RestMethod -Uri "http://localhost:5500/vehicle" -Method Post

# Get customers information
Invoke-RestMethod -Uri "http://localhost:5500/customers" -Method Get