-- IMPORTANT: to prevent a security issue, the following initial platform user's password
-- needs to be changed after the postgres service container is created.
-- The password hash params are password and a random salt value.
INSERT INTO refactor_platform.users (
    display_name, 
    email, 
    first_name, 
    last_name, 
    github_profile_url, 
    github_username, 
    password
) 
VALUES (
    'Admin',
    'admin@refactorcoach.com',
    'Admin',
    'User',
    '#',
    '',
    '$argon2id$v=19$m=19456,t=2,p=1$x4aqeh1xRaYsgeo0I5kB3A$gQz+ARyx7e6WjCNyNkOXNYFnLnc1eWqHXY+ASmJ7PDM'
);