FROM postgres:14.1
COPY .. ./postgres_spice
WORKDIR /postgres_spice

RUN chmod +x command_ref.sh
RUN ./command_ref.sh
