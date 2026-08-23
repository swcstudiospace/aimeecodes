# Self-hosted ITSM / helpdesk shortlist

GitHub activity checked ~2026-08 (stars approximate).

## Full ITSM / ITIL-ish

| Tool | Stars | License | Notes |
|------|------:|---------|-------|
| GLPI | ~6.2k | GPL-3.0 | Assets + tickets + licenses; best single-box ITSM |
| iTop | ~1.2k | AGPL-3.0 | CMDB-first ITIL |
| Request Tracker | ~1.1k | GPL-2.0 | Mail-driven enterprise tickets; Perl stack |

## Helpdesk / shared inbox

| Tool | Stars | License | Notes |
|------|------:|---------|-------|
| Zammad | ~5.9k | AGPL-3.0 | Best modern UX; heavy (ES+PG+Redis) |
| FreeScout | ~4.5k | AGPL-3.0 | Light Help Scout alternative |
| osTicket | ~3.9k | GPL-2.0 | Classic simple tickets |
| UVdesk | ~19k skeleton | OSL-3.0 | Symfony; feature-rich; medium RAM |

## Assets / inventory (pair with tickets)

| Tool | Notes |
|------|-------|
| Snipe-IT | Best open ITAM / licenses |
| InvenTree | Parts/warehouse more than ITAM |
| NetBox | Network source of truth, not a desk |

## Adjacent

Cachet (status), LibreNMS (monitoring), BookStack/Wiki.js (KB), Passbolt (secrets).

## This host pick guidance

- Eng already: Linear + Hermes Kanban (+ optional Vikunja human UI)
- Prefer **one** medium or light ITSM; do not stack Zammad + UVdesk
- Installed: **UVdesk** on `:8082` — see `uvdesk-host-install.md`
