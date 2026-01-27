pub static ENV_EXAMPLE: &[u8] = 
    include_bytes!("../assets/env.example");

pub static DJANGO_DEV: &[u8] =
    include_bytes!("../assets/django/settings/dev.py");

pub static DJANGO_PROD: &[u8] =
    include_bytes!("../assets/django/settings/prod.py");

pub static MAKEFILE: &[u8] =
    include_bytes!("../assets/Makefile");

