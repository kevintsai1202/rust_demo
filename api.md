# API 文件 - Rust Demo API

## 基礎資訊
- **Base URL**: `http://localhost:3000`
- **Content-Type**: `application/json`

## Endpoints

### 1. 根目錄
- **URL**: `/`
- **Method**: `GET`
- **Description**: 檢查伺服器是否運作
- **Response**:
    - Status: 200 OK
    - Body (Text): "Hello, Rust API!"

### 2. 取得使用者列表
- **URL**: `/users`
- **Method**: `GET`
- **Description**: 回傳所有使用者
- **Response**:
    - Status: 200 OK
    - Body (JSON):
      ```json
      [
        {
          "id": 1,
          "username": "alice",
          "email": "alice@example.com"
        }
      ]
      ```

### 3. 建立使用者
- **URL**: `/users`
- **Method**: `POST`
- **Description**: 建立新的使用者
- **Request Body** (JSON):
    ```json
    {
      "username": "bob",
      "email": "bob@example.com"
    }
    ```
- **Response**:
    - Status: 201 Created
    - Body (JSON):
      ```json
      {
        "id": 1337,
        "username": "bob",
        "email": "bob@example.com"
      }
      ```
