# 🐻 Tanukeys

[English 🇬🇧](../../README.md) [Español 🇪🇸](./ES.md) [日本語 🇯🇵](./JP.md)

Servicio para almacenar claves públicas de usuarios con diversos propósitos, como la verificación de firmas de mensajes, el cifrado de información para un usuario específico, el almacenamiento de direcciones de monederos de criptomonedas y la simplificación del intercambio de claves entre dispositivos. El servicio también admite algoritmos criptográficos post-cuánticos para garantizar la seguridad a largo plazo.  

Tanukeys está diseñado como un **sistema federado**, lo que permite a las instancias interoperar recuperando y verificando claves públicas de otras instancias de Tanukeys. Esto facilita el intercambio de claves, las actualizaciones automáticas mediante suscripciones entre instancias y un modelo de confianza descentralizado que mejora la seguridad y la fiabilidad.  


## Características

- [ ] **Almacenamiento de Claves Públicas**: Guarda claves públicas asociadas con usuarios o dispositivos individuales.
- [ ] **Verificación de Firmas de Mensajes**: Recupera claves públicas almacenadas para verificar mensajes firmados.
- [ ] **Cifrado de Datos**: Usa las claves públicas almacenadas para cifrar datos destinados a un usuario específico.
- [ ] **Monederos de Criptomonedas**: Administra claves públicas de diferentes monederos de criptomonedas.
- [ ] **Soporte para Criptografía Post-Cuántica**: Permite almacenar claves de algoritmos post-cuánticos para garantizar la seguridad a largo plazo.
- [ ] **Recuperación Federada de Claves**: Permite importar y verificar claves públicas de instancias externas de Tanukeys, posibilitando la interoperabilidad en redes federadas.
- [ ] **Sistema de Suscripción Federado**: Permite que las instancias se suscriban a otras instancias de Tanukeys para sincronizarse y actualizar automáticamente las claves públicas cuando cambien.
- [ ] **Subclaves Firmadas**: Permite a los usuarios crear subclaves firmadas que hacen referencia a una clave maestra, asegurando una gestión jerárquica de claves.


## Tecnologías Utilizadas

- **Backend**: Desarrollado en Rust.
- **Base de Datos**: PostgreSQL para la persistencia de claves públicas e información de usuarios.


## Licencia

Este proyecto está licenciado bajo la Licencia Pública General Affero de GNU (AGPL) v3.0. Esta licencia garantiza que cualquier modificación del software, ya sea de uso privado o público, debe compartirse bajo la misma licencia. Para más detalles, consulta el archivo LICENSE o visita [https://www.gnu.org/licenses/agpl-3.0.html](https://www.gnu.org/licenses/agpl-3.0.html).
