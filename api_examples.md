# Vending Records API Examples

## Health Check Endpoints

### Basic Health Check
```bash
curl -X GET http://127.0.0.1:8092/health
```

### Database Readiness Check
```bash
curl -X GET http://127.0.0.1:8092/health/ready
```

## Vending Records Endpoints

### Get Vending Records
```bash
# Get all records (defaults to last 30 days)
curl -X GET http://127.0.0.1:8092/api/vending-records

# Get records with date-only format (simple)
curl -X GET "http://127.0.0.1:8092/api/vending-records?start_date=2024-01-01&end_date=2024-12-31"

# Get records with full datetime format (precise)
curl -X GET "http://127.0.0.1:8092/api/vending-records?start_date=2024-01-01T00:00:00Z&end_date=2024-12-31T23:59:59Z"
```

### Get Vending Summary
```bash
# Get summary for last 30 days (default)
curl -X GET http://127.0.0.1:8092/api/vending-records/summary

# Get summary with date-only format (simple)
curl -X GET "http://127.0.0.1:8092/api/vending-records/summary?start_date=2024-01-01&end_date=2024-01-31"

# Get summary with full datetime format (precise)
curl -X GET "http://127.0.0.1:8092/api/vending-records/summary?start_date=2024-01-01T00:00:00Z&end_date=2024-01-31T23:59:59Z"

# Pretty formatted with jq
curl -s "http://127.0.0.1:8092/api/vending-records/summary?start_date=2024-01-01&end_date=2024-01-31" | jq
```

## Response Format

All API responses follow this structure:

```json
{
  "success": true,
  "message": "Description of the result",
  "data": {
    // Response data here
  }
}
```

### Successful Response Example (Records):
```json
{
  "success": true,
  "message": "Retrieved 5 vending records",
  "data": [
    {
      "id": "507f1f77bcf86cd799439011",
      "timestamp": "2024-01-15T10:30:00Z",
      "meter_number": "MTR001",
      "address": "123 Main St",
      "community": "Downtown",
      "customer_name": "John Doe",
      "token": "TKN123456",
      "tariff": 0.15,
      "amount": 25.00,
      "kwh": 166.67,
      "user_id": "user123",
      "vending_station": "Station A",
      "fixed_charge": 2.50,
      "transaction_id": "TXN789012",
      "remaining_credit": 75.50
    }
  ]
}
```

### Successful Summary Response Example:
```json
{
  "success": true,
  "message": "Retrieved vending summary for period 2024-01-01 to 2024-01-31 (150 total transactions)",
  "data": {
    "total_transactions": 150,
    "total_amount": 12500.75,
    "total_kwh": 8333.50,
    "period_start": "2024-01-01",
    "period_end": "2024-01-31",
    "vending_station_summaries": [
      {
        "vending_station": "Station A",
        "total_transactions": 75,
        "total_amount": 6250.25,
        "total_kwh": 4166.75,
        "period_start": "2024-01-01",
        "period_end": "2024-01-31",
        "daily_summaries": [
          {
            "date": "2024-01-01",
            "total_transactions": 5,
            "total_amount": 425.50,
            "total_kwh": 283.67
          },
          {
            "date": "2024-01-02",
            "total_transactions": 8,
            "total_amount": 680.80,
            "total_kwh": 453.87
          }
        ]
      },
      {
        "vending_station": "Station B",
        "total_transactions": 75,
        "total_amount": 6250.50,
        "total_kwh": 4166.75,
        "period_start": "2024-01-01",
        "period_end": "2024-01-31",
        "daily_summaries": [
          {
            "date": "2024-01-01",
            "total_transactions": 3,
            "total_amount": 255.30,
            "total_kwh": 170.20
          }
        ]
      }
    ]
  }
}
```

### Error Response Example:
```json
{
  "success": false,
  "message": "Invalid record ID format",
  "data": null
}
```

## Query Parameters

### Date Range Queries
Both endpoints support flexible date formats:

**Simple date format (recommended):**
- `start_date=2024-01-01` - Date only (time set to 00:00:00)
- `end_date=2024-12-31` - Date only (time set to 23:59:59.999)

**Full datetime format (precise):**
- `start_date=2024-01-01T00:00:00Z` - Full ISO 8601 datetime
- `end_date=2024-12-31T23:59:59Z` - Full ISO 8601 datetime

**Defaults:**
- If no dates provided: Returns last 30 days
- `start_date` defaults to 30 days ago
- `end_date` defaults to now

## HTTP Status Codes

- `200 OK`: Successful request
- `400 Bad Request`: Invalid request format or parameters  
- `500 Internal Server Error`: Server error
- `503 Service Unavailable`: Database connectivity issues
