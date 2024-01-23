FROM eclipse-temurin:21-jdk-jammy as builder
RUN apt update && apt -y install git
WORKDIR /app
RUN git clone https://github.com/gunnarmorling/1brc.git
WORKDIR /app/1brc
RUN ./mvnw clean verify
RUN ./create_measurements.sh 10000
RUN mv measurements.txt measurements-10000.txt
RUN ./create_measurements.sh 1000000000
RUN mv measurements.txt measurements-1000000000.txt

FROM scratch
COPY --from=builder /app/1brc/measurements-10000.txt .
COPY --from=builder /app/1brc/measurements-1000000000.txt .
