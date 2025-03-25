# Rust-Employee-API
An API with CRUD function that connects to an Employee database.
Made using Rust and using Axum as web app framework.

Also features JWT authentication (secret key stored at Environment Variables) and API rate limiting.
To be added:
- SODIUM encryption of json output, decryption of said data from PHP layer.

Currently, the code includes encryption, but will be removed for testing in this branch.

To run the API, these Pre-requisites are needed:
1. XAMPP software with MySQL and database initialized. Database should run before launching the API
Database can be generated from included employee.sql file.
2. Rust installed.
3. JWT secret key (64 Bytes, generate from: https://jwtsecret.com/generate). Then, set to your OS' Environment Variables setting under JWT_SECRET.
4. JWT web token:
PAYLOAD:DATA format
{
  "sub": "your_username", // Set with any username you want
  "exp": 1742803200 // set to at least 1 hour or more from current time. Use https://www.unixtimestamp.com/index.php for unix time stamp format)
}
VERIFY SIGNATURE:
- Paste the generated secret key to obtain the encoded JWT token.


Once downloaded, open terminal/cmd, navigate to directory containing the files, and then run 'cargo build'
Then, run 'cargo run' to launch the server. 
