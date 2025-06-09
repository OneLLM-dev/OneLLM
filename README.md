# OneAI Implementation Roadmap
This roadmap outlines the steps needed to enhance your OneAI platform with SRP2 authentication, rate limiting solutions, secure session management, and additional features like Gemini pricing. The plan is structured to help you launch by June 30th.

## Current System Overview
- Authentication : Currently using Argon2 password hashing with email/password login
- Database : PostgreSQL with a simple users table (email, password, apikey, balance)
- API Access : Bearer token authentication using API keys
- Pricing : Models defined for OpenAI, Claude, DeepSeek, and Gemini (without specific pricing)
## Week 1 (June 10-16): Database & Authentication Enhancements
### 1. Database Schema Updates
- Update users table to support SRP2 authentication
  - Add columns for salt, verifier, and SRP session data
  - Create migration script
```sql
ALTER TABLE users 
ADD COLUMN srp_salt VARCHAR,
ADD COLUMN srp_verifier VARCHAR,
ADD COLUMN last_login TIMESTAMP,
ADD COLUMN failed_attempts INTEGER DEFAULT 0,
ADD COLUMN locked_until TIMESTAMP;
```
- Create sessions table for managing user sessions
```sql
CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_email VARCHAR NOT NULL REFERENCES users
    (email),
    session_token VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    ip_address VARCHAR,
    user_agent VARCHAR
);

CREATE INDEX sessions_token ON sessions
(session_token);
CREATE INDEX sessions_user ON sessions(user_email);
```
### 2. SRP2 Implementation
- Add srp-rs crate to dependencies
```toml
# Add to Cargo.toml
srp = "0.6.0"
```
- Create SRP utility module ( src/srp.rs )
  - Implement SRP registration flow
  - Implement SRP authentication flow
  - Update existing auth functions to use SRP
## Week 2 (June 17-23): Session Management & Rate Limiting
### 1. Session Management
- Implement session creation on successful login
- Add session validation middleware
- Create session refresh mechanism
- Implement secure logout functionality
### 2. Rate Limiting Solution
- Add rate limiting crate to dependencies
```toml
# Add to Cargo.toml
axum-extra = { version = "0.9.0", features = 
["rate-limit"] }
```
- Implement tiered rate limiting
  
  - IP-based rate limiting for unauthenticated requests
  - User-based rate limiting for authenticated requests
  - Different limits for different API endpoints
- Create custom rate limit store with PostgreSQL
  
  - Track request counts in database
  - Allow for adjustable limits based on user subscription
## Week 3 (June 24-30): Pricing Updates & Final Integration
### 1. Gemini Pricing Implementation
- Update Gemini pricing in pricing.rs
```rs
async fn price(&self) -> f32 {
    match self {
        GeminiModel::Gem25_FlashPrev => return 0.35,
        GeminiModel::Gem25ProPrev => return 3.50,
        GeminiModel::Gem25FlashNativeAudio => return 
        0.50,
    }
}
```
- Update pricing.html to display accurate Gemini pricing
### 2. Integration & Testing
- Integrate SRP2 with frontend login/signup forms
- Test session management across different browsers
- Verify rate limiting functionality
- Perform security audit
  - Check for SQL injection vulnerabilities
  - Verify HTTPS configuration
  - Test authentication flows
### 3. Documentation & Deployment Preparation
- Update API documentation
- Create user guide for authentication
- Prepare deployment scripts
- Set up monitoring for rate limits and authentication attempts
## Implementation Details
### SRP2 Implementation
The Secure Remote Password protocol allows secure password-based authentication without sending the password over the network:

1. Registration Process :
   
   - Client generates salt and verifier from password
   - Server stores salt and verifier (not the password)
2. Authentication Process :
   
   - Client and server exchange proofs without revealing password
   - Mutual authentication is achieved
### Rate Limiting Strategy
Instead of blocking users completely, implement a tiered approach:

1. Free Tier : 60 requests per minute
2. Basic Tier : 300 requests per minute
3. Premium Tier : 1000 requests per minute
Implement exponential backoff for excessive requests rather than hard blocking.

### Session Management
1. Session Creation : Generate secure random tokens stored in HTTP-only cookies
2. Session Validation : Middleware to check session validity on protected routes
3. Session Expiry : Auto-expire sessions after inactivity (configurable)
4. Session Refresh : Implement token rotation for long-lived sessions
## Security Considerations
- Use HTTPS for all communications
- Implement CSRF protection
- Store session tokens securely (HTTP-only, Secure cookies)
- Implement proper error handling without leaking sensitive information
- Add brute force protection with temporary account lockouts
## Launch Checklist
- All database migrations tested and ready
- SRP2 authentication fully implemented
- Session management working correctly
- Rate limiting configured and tested
- Gemini pricing updated
- Security audit completed
- Documentation updated
- Deployment scripts prepared
By following this roadmap, you should be able to launch a secure, robust version of OneAI by June 30th with all the requested features implemented.