import MiniSearch from 'minisearch'

const miniSearch = new MiniSearch({
  fields: ['title', 'content'],
  storeFields: ['title', 'url']
})

async function loadPosts() {
  const response = await fetch('/search-index.json')
  const posts = await response.json()
  miniSearch.addAll(posts)
}

function handleSearch(e) {
  const query = e.target.value
  if (!query) {
    hideResults()
    return
  }
  
  const results = miniSearch.search(query)
  showResults(results)
}

document.addEventListener('DOMContentLoaded', () => {
  loadPosts()
  const searchInput = document.querySelector('input[type="search"]')
  searchInput.addEventListener('input', handleSearch)
})
