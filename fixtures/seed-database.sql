insert into users (id,
                   first_name,
                   last_name,
                   email,
                   created_at,
                   updated_at,
                   hash,
                   salt,
                   password_reset_at,
                   password_reset_token,
                   confirmed_at,
                   confirmation_sent_at,
                   confirmation_token)
VALUES (1,
        'Logan',
        'Keenan',
        'login-success@gmail.com',
        '2020-04-27 21:10:30.840223',
        null,
        E'\\x5159FA23F83F1D7836C524D607CC901AE8FEC4E0754B96098EB8F9DCE9D4B492',
        '%Bp)U3vk0ZQCsuj)fwPk@kEM5*lcLvLMOMAaYr$32dzG9j$#g0$WjR%fooj$5NY4qP9E%c2paBVBnMK$4vcYFW!IdmV@bXx@IZ3@jWOh~M3qaaKuXjAs6zEePIn$GeSJ',
        null,
        null,
        '2020-04-27 21:10:30.840512',
        '2020-04-27 21:10:30.840512',
        'f0168e2a-0162-46e0-a433-0e9a130a31cd');

insert into users (id,
                   first_name,
                   last_name,
                   email,
                   created_at,
                   updated_at,
                   hash,
                   salt,
                   password_reset_at,
                   password_reset_token,
                   confirmed_at,
                   confirmation_sent_at,
                   confirmation_token)
VALUES (2,
        'Logan',
        'Keenan',
        'login-unconfirmed@gmail.com',
        '2020-04-27 21:10:30.840223',
        null,
        E'\\x5159FA23F83F1D7836C524D607CC901AE8FEC4E0754B96098EB8F9DCE9D4B492',
        '%Bp)U3vk0ZQCsuj)fwPk@kEM5*lcLvLMOMAaYr$32dzG9j$#g0$WjR%fooj$5NY4qP9E%c2paBVBnMK$4vcYFW!IdmV@bXx@IZ3@jWOh~M3qaaKuXjAs6zEePIn$GeSJ',
        null,
        null,
        null,
        '2020-04-27 21:10:30.840512',
        'f0168e2a-0162-46e0-a433-0e9a130a31ce');

insert into users (id,
                   first_name,
                   last_name,
                   email,
                   created_at,
                   updated_at,
                   hash,
                   salt,
                   password_reset_at,
                   password_reset_token,
                   confirmed_at,
                   confirmation_sent_at,
                   confirmation_token)
VALUES (3,
        'Logan',
        'Keenan',
        'success-confirmation@gmail.com',
        '2020-04-27 21:10:30.840223',
        null,
        E'\\x5159FA23F83F1D7836C524D607CC901AE8FEC4E0754B96098EB8F9DCE9D4B492',
        '%Bp)U3vk0ZQCsuj)fwPk@kEM5*lcLvLMOMAaYr$32dzG9j$#g0$WjR%fooj$5NY4qP9E%c2paBVBnMK$4vcYFW!IdmV@bXx@IZ3@jWOh~M3qaaKuXjAs6zEePIn$GeSJ',
        null,
        null,
        null,
        current_timestamp,
        'f0168e2a-0162-46e0-a433-0e9a130a31cf');

insert into users (id,
                   first_name,
                   last_name,
                   email,
                   created_at,
                   updated_at,
                   hash,
                   salt,
                   password_reset_at,
                   password_reset_token,
                   confirmed_at,
                   confirmation_sent_at,
                   confirmation_token)
VALUES (4,
        'Logan',
        'Keenan',
        'expired-confirmed@gmail.com',
        '2020-04-27 21:10:30.840223',
        null,
        E'\\x5159FA23F83F1D7836C524D607CC901AE8FEC4E0754B96098EB8F9DCE9D4B492',
        '%Bp)U3vk0ZQCsuj)fwPk@kEM5*lcLvLMOMAaYr$32dzG9j$#g0$WjR%fooj$5NY4qP9E%c2paBVBnMK$4vcYFW!IdmV@bXx@IZ3@jWOh~M3qaaKuXjAs6zEePIn$GeSJ',
        null,
        null,
        null,
        (current_timestamp - INTERVAL '1 day'),
        'f0168e2a-0162-46e0-a433-0e9a130a32cf');

insert into users (id,
                   first_name,
                   last_name,
                   email,
                   created_at,
                   updated_at,
                   hash,
                   salt,
                   password_reset_at,
                   password_reset_token,
                   confirmed_at,
                   confirmation_sent_at,
                   confirmation_token)
VALUES (5,
        'Logan',
        'Keenan',
        'resend-confirmed@gmail.com',
        '2020-04-27 21:10:30.840223',
        null,
        E'\\x5159FA23F83F1D7836C524D607CC901AE8FEC4E0754B96098EB8F9DCE9D4B492',
        '%Bp)U3vk0ZQCsuj)fwPk@kEM5*lcLvLMOMAaYr$32dzG9j$#g0$WjR%fooj$5NY4qP9E%c2paBVBnMK$4vcYFW!IdmV@bXx@IZ3@jWOh~M3qaaKuXjAs6zEePIn$GeSJ',
        null,
        null,
        null,
        current_timestamp,
        'f0168e2a-0162-46e0-a433-0e9a130a33cf');




