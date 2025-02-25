# 🐻 Tanukeys

[English 🇬🇧](/README.md) [Español 🇪🇸](./ES.md) [日本語 🇯🇵](./JP.md)

Servicio para almacenar claves públicas de usuarios con diversos propósitos, tales como la verificación de firmas de mensajes, el cifrado de información para un usuario específico, el almacenamiento de direcciones de billeteras de criptomonedas y la simplificación del intercambio de claves entre dispositivos. El servicio también es compatible con algoritmos criptográficos post-cuánticos para garantizar su seguridad a largo plazo.

## Características

- **Almacenamiento de Claves Públicas**: Guarda claves públicas asociadas a usuarios o dispositivos individuales.
- **Verificación de Firmas de Mensajes**: Recupera las claves públicas almacenadas para verificar mensajes firmados.
- **Cifrado de Datos**: Utiliza las claves públicas almacenadas para cifrar datos destinados a un usuario específico.
- **Billeteras de Criptomonedas**: Gestiona claves públicas para diversas billeteras de criptomonedas.
- **Simplificación del Intercambio de Claves**: Facilita el intercambio de claves públicas entre dispositivos.
- **Soporte para Criptografía Post-Cuántica**: Incluye la capacidad de almacenar claves para algoritmos post-cuánticos, garantizando la seguridad a largo plazo.

## Tecnología Utilizada

- **Backend**: Desarrollado en Rust.
- **Base de Datos**: PostgreSQL para la persistencia de claves públicas e información de usuarios.

## Licencia

Este proyecto está licenciado bajo la Licencia Pública General Affero de GNU (AGPL) v3.0. Esta licencia garantiza que cualquier modificación al software, ya sea de uso privado o público, debe compartirse bajo la misma licencia. Para más detalles, consulta el archivo LICENSE o visita [https://www.gnu.org/licenses/agpl-3.0.html](https://www.gnu.org/licenses/agpl-3.0.html).
