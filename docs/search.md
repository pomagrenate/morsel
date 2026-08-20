# Search Documentation

## Overview

Morsel provides powerful search capabilities for your clipboard history, including fuzzy matching, content type filtering, and ranking.

## Search Features

### Fuzzy Search

Fuzzy search allows you to find items even with typos or partial matches:

```bash
morsel search "gthub"  # Matches "github"
morsel search "api"     # Matches "api_key", "api_endpoint"
```

Fuzzy search is enabled by default. Disable with `--exact`:

```bash
morsel search --exact "github"
```

### Case Sensitivity

Search is case-insensitive by default:

```bash
morsel search "GITHUB"  # Matches "github", "GitHub", "GITHUB"
```

Enable case-sensitive search:

```bash
morsel search --case-sensitive "GitHub"
```

### Content Type Filtering

Filter by content type:

```bash
morsel search --content-type url "example"
morsel search --content-type email "test"
morsel search --content-type code "function"
morsel search --content-type sql "SELECT"
morsel search --content-type json "key"
```

Available content types:
- `text` - Plain text
- `url` - URLs
- `email` - Email addresses
- `code` - Code snippets
- `json` - JSON data
- `sql` - SQL queries
- `markdown` - Markdown
- `yaml` - YAML
- `xml` - XML

### Tag Filtering

Search within tagged items:

```bash
morsel search --tag work "api"
morsel search --tag important "password"
```

### Favorite Filtering

Search only favorites:

```bash
morsel search --favorites "config"
```

### Limiting Results

Limit the number of results:

```bash
morsel search --limit 5 "github"
```

## Search Query Syntax

### Basic Search

```bash
morsel search "query"
```

### Multiple Terms

Search for multiple terms (AND logic):

```bash
morsel search "github api"
```

### Exact Phrase

Use quotes for exact phrase matching:

```bash
morsel search '"api key"'
```

### Negation

Exclude terms (NOT logic):

```bash
morsel search "github -api"
```

### OR Logic

Use `|` for OR logic:

```bash
morsel search "github|gitlab"
```

## Ranking

Results are ranked by:
1. **Relevance score** - How well the query matches
2. **Recency** - More recent items rank higher
3. **Usage frequency** - Frequently used items rank higher
4. **Favorite status** - Favorites rank higher

### Score Calculation

The score is calculated based on:
- Exact match bonus
- Fuzzy match penalty
- Position of match (earlier = better)
- Content length (shorter = better for exact matches)

## Search Configuration

Configure default search behavior in `config.toml`:

```toml
[search]
case_insensitive = true
fuzzy = true
max_results = 100
```

## Search Performance

### Indexing

Search uses an in-memory index for fast lookups:
- Items indexed on storage
- Index updated on insert/delete
- O(1) lookup for exact matches
- O(n) for fuzzy matches (n = index size)

### Optimization Tips

1. **Limit results** - Use `--limit` for faster queries
2. **Use filters** - Filter by content type or tags to reduce search space
3. **Exact matches** - Use `--exact` for faster searches
4. **Case sensitivity** - Case-sensitive is slightly faster

### Large Datasets

For datasets > 10,000 items:
- Consider increasing `max_results` for comprehensive results
- Use content type filters to narrow search
- Use tag filters for organized searches

## Advanced Usage

### Search and Copy

Search and copy the first result:

```bash
morsel search "api" --copy
```

### Search by ID Range

Search within a specific time range:

```bash
morsel search --older-than 7d "github"
morsel search --newer-than 1d "config"
```

### Search with Regex

Use regex patterns:

```bash
morsel search --regex "api_[a-z]+"
```

### Search in Collections

Search within a specific collection:

```bash
morsel search --collection "Project X" "api"
```

## Search Examples

### Find URLs

```bash
morsel search --content-type url "github"
```

### Find Code Snippets

```bash
morsel search --content-type code "function"
```

### Find Email Addresses

```bash
morsel search --content-type email "company"
```

### Find SQL Queries

```bash
morsel search --content-type sql "users"
```

### Find JSON Data

```bash
morsel search --content-type json "token"
```

### Find Recent Items

```bash
morsel search --newer-than 1h "config"
```

### Find Old Items

```bash
morsel search --older-than 30d "legacy"
```

### Find in Tags

```bash
morsel search --tag work "meeting"
```

### Find Favorites

```bash
morsel search --favorites "important"
```

## Troubleshooting

### No Results

**Problem:** Search returns no results

**Solutions:**
- Check spelling
- Try fuzzy search (default)
- Try broader search terms
- Check content type filter
- Check tag filter

### Too Many Results

**Problem:** Search returns too many results

**Solutions:**
- Use `--limit` to reduce results
- Add more specific terms
- Use content type filter
- Use tag filter
- Use exact match with `--exact`

### Slow Search

**Problem:** Search is slow

**Solutions:**
- Use `--limit` to reduce results
- Use content type filter
- Use exact match with `--exact`
- Consider cleanup to reduce database size

## Search Best Practices

1. **Use specific terms** - More specific = better results
2. **Leverage content types** - Filter by type when possible
3. **Organize with tags** - Tags make searching easier
4. **Mark favorites** - Favorites rank higher
5. **Use limits** - Limit results for faster searches
6. **Regular cleanup** - Remove old items to keep search fast
