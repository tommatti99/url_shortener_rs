# Rust URL Shortener

Its available at https://url-shortener-rs.up.railway.app/#/

## Stack
  * Frontend: VueJs Quasar
  * Backend: Rust Rocker and Diesel
  * Database: PostgresSQL
  * Deploy: Docker

## Environment Variables

### Backend
  * DATABASE_URL: the postgres database path. example: postgresql://username:password@host:port/database
    
### Frontend
  * VITE_BASE_URL: the backend base path.
  * VITE_CREATE_LINK_API: path for api thats create a new short link. default: /api/new_short_link
  * VITE_GET_FULL_LINK_API: path for api thats get a full link from a short one. default: /api/gt

## Screenshot

![Screenshot do Projeto](frontend/screenshot/screenshot1.png)
![Screenshot do Projeto](frontend/screenshot/screenshot2.png)

