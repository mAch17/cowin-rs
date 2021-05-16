# cowin-rs
COWIN-RS is a command line tool that automates searching for free vaccine slots in India. It polls the COWIN APIs [ www.cowin.gov.in ] every 5 minutes (or as you configure) and tries to find out any free vaccine slots in your pincode or district.
Its written in Rust and thus can be downloaded as an executable. No installation needed. Just download on your computer and trigger using command line as cowin-rs --help and cowin-rs trackIn --help to see the options provided. Uses callmebot API [only if you want] to send you a whatsapp message as soon as a slot is available.

