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

## Vending Records Endpoint

### Get Vending Records
```bash
# Get all records (defaults to last 30 days)
curl -X GET http://127.0.0.1:8092/api/vending-records

# Get records with specific date range
curl -X GET "http://127.0.0.1:8092/api/vending-records?start_date=2024-01-01T00:00:00Z&end_date=2024-12-31T23:59:59Z"
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

### Successful Response Example:
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
- `start_date`: ISO 8601 format (e.g., "2024-01-01T00:00:00Z") - defaults to 30 days ago
- `end_date`: ISO 8601 format (e.g., "2024-12-31T23:59:59Z") - defaults to now

## HTTP Status Codes

- `200 OK`: Successful request
- `400 Bad Request`: Invalid request format or parameters  
- `500 Internal Server Error`: Server error
- `503 Service Unavailable`: Database connectivity issues
