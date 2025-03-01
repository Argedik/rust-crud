# 1. Aşama: Derleme
FROM rust:1.85 AS builder

WORKDIR /app
# Proje dosyalarını kopyala
COPY . .

# Release modda derleme
RUN cargo build --release

# 2. Aşama: Daha ufak bir tabanda sadece ikili dosyaları kullan
FROM debian:bullseye-slim AS runtime

WORKDIR /app
# Builder aşamasında oluşan binary dosyaları kopyala
COPY --from=builder /app/target/release/create /app/create
COPY --from=builder /app/target/release/read   /app/read
COPY --from=builder /app/target/release/update /app/update
COPY --from=builder /app/target/release/delete /app/delete

# Veritabanı dosyamız (çalışma anında da yaratılabilir)
# COPY database.json /app/database.json   # İsterseniz, eğer bir başlangıç verisi gerekiyorsa

# Varsayılan olarak "create" komutunu çalıştırmak isterseniz:
CMD ["/app/create"]
