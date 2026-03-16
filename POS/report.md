# POS UI Full Audit Report

Date: 2026-03-16
Scope: Admin dashboard tabs and supporting UI in `POS/src/ui/pages/admin.rs`, related styles in `POS/src/ui/pages/admin.css`, chart component in `POS/src/ui/components/dashborads/sales_trend.rs`.

## Executive Summary
The admin UI is visually strong and consistent, with a well-structured dashboard and usable data tables. The main gaps are in action realism, cross-tab data consistency, and missing operational workflows (editing, filtering, exporting, and drill‑downs). Most tabs render static or placeholder actions and lack validation, empty-state guidance, and consistent KPI calculation logic.

## High Priority Issues
1. Actions are non-functional placeholders in most tabs (Edit, Delete, Void, View, Add). Users cannot complete core workflows.
2. Mixed metrics sources across tabs lead to inconsistent numbers (sales, products, users), especially for “completed vs paid” states.
3. Several tabs rely on generic placeholders rather than real operational data (Settings confirmation, Payments settlement notes, Shifts coverage notes).
4. No filter/search/sort on large tables (Products, Sales, Staff, Customers), which will not scale.

## Dashboard Tab Audit
Status: Updated, real metrics partially wired.

Findings:
- Sales chart now uses real daily aggregation from `sales.json` and `sale_items.json`.
- Category donut now uses revenue by category, but category labels in the list show revenue while the label text still implies “items.”
- Top alerts are static and not connected to real data.
- “Last Activity” still uses sales and shows cashier id only.
- Product Performance is now based on `sale_items.json` revenue, but columns and labels should align with “Sold / Stock / Avg Price / Revenue.”

Recommendations:
1. Rename “Top Categories” list label to reflect revenue (“$ total” or “Revenue”).
2. Wire alerts to actual conditions (low stock, out-of-stock, pending payments).
3. Add quick filters (“Last 7d / 30d / All time”) for chart with real filtering logic.
4. Add trend deltas using period-over-period calculations for KPIs.

## Products Tab Audit
Status: Functional table UI, add modal is placeholder.

Findings:
- Table renders from JSON but no search, sort, or pagination.
- Add Product modal has no validation beyond required name/barcode.
- Image upload is not implemented and no preview storage pipeline exists.
- Duplicate summary blocks are present but not connected to actions.

Recommendations:
1. Add search and category filter in header.
2. Add low-stock filter and a stock reorder suggestion block.
3. Provide inline “Edit” modal with validation and data binding.
4. Add bulk actions (export, import, price update).

## Sales Tab Audit
Status: Core table exists, actions are placeholders.

Findings:
- Completed vs pending uses `status == "completed"` but `sales.json` uses `paid`.
- No drill-down view for line items per sale.
- “Sale Items” panel uses generic keys without currency formatting.

Recommendations:
1. Align status logic with data (`paid` vs `completed`) or normalize in data layer.
2. Add expandable row or side panel with `sale_items.json` detail.
3. Add filters by cashier, payment method, date range.

## Staff Tab Audit
Status: Table and add modal exist but actions are placeholders.

Findings:
- Role and status display is present but no edit flow.
- Profile image upload not wired.
- No staff performance metrics (sales per staff, shift attendance).

Recommendations:
1. Add edit modal with role/status toggle.
2. Connect staff to sales and shifts to show performance KPIs.

## Customers Tab Audit
Status: Partial with insights placeholder.

Findings:
- Uses extra JSON for customers, no detail or segmentation.
- No loyalty, frequency, or top customer metrics.

Recommendations:
1. Add “Top Customers” and “Repeat Rate” metrics.
2. Add customer profile drawer with history.

## Inventory Tab Audit
Status: Basic operational view.

Findings:
- Recent movements rely on `inventory_movements.json` but don’t connect to product data.
- Categories panel is static and doesn’t show counts or value.

Recommendations:
1. Add product stock valuation per category.
2. Flag inventory movements that drop below reorder threshold.

## Payments Tab Audit
Status: Basic list, no operational workflows.

Findings:
- Payments are listed but no link to sales or receipts.
- “Settlement Notes” is placeholder.

Recommendations:
1. Add payment method breakdown and daily totals.
2. Add reconciliation status and export for accounting.

## Access Control Tab Audit
Status: Good baseline for visibility, no workflows.

Findings:
- Roles/permissions lists are static, no assignment actions.

Recommendations:
1. Add role management modal and permission checklist UI.
2. Add user-role assignment UI with validation.

## Suppliers Tab Audit
Status: Simple list view.

Findings:
- No supplier performance metrics.

Recommendations:
1. Add last delivery, average lead time, active product count.

## Shifts Tab Audit
Status: Basic list.

Findings:
- No tie-in with staff or sales per shift.
- Coverage notes are placeholder.

Recommendations:
1. Show shift totals: sales, transactions, staff count.
2. Add shift approval and closeout summary.

## Settings Tab Audit
Status: Static settings.

Findings:
- Settings are not persisted.
- Currency does not apply across UI formatting.

Recommendations:
1. Store settings to JSON and apply currency formatting globally.
2. Add business address, tax rate, and receipt footer configuration.

## Cross-Cutting UI Recommendations
1. Add global search and quick filters across all data tabs.
2. Standardize empty state wording and actions.
3. Add consistent KPI cards per tab header with trend deltas.
4. Add export buttons (CSV/PDF) on table-heavy tabs.

## Data Consistency Checklist
1. Normalize sales status values (`paid`, `completed`, `pending`).
2. Ensure sale items always link to products via `product_id`.
3. Add optional `cost_price` to products for profit analytics.
4. Use a shared formatter for currency and dates.

## Next Implementation Plan (Suggested)
1. Fix Sales status mapping and add sale detail view.
2. Add Product edit flow and inventory thresholds.
3. Add payments breakdown and reconciliation export.
4. Persist settings and apply currency globally.

