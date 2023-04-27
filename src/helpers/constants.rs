pub const ANILIST_ANIME_QUERY: &'static str = "
query($search: String, $page: Int = 1, $per_page: Int = 10) {
    Page(page: $page, perPage: $per_page) {
        pageInfo {
            total
            currentPage
            lastPage
        }
        media(search: $search, type: ANIME, sort: POPULARITY_DESC) {
            title {
                romaji
                english
                native
            },
            description(asHtml: false)
            siteUrl
            bannerImage
            coverImage {
                large
            },
            startDate {
                year
                month
                day
            },
            endDate {
                year
                month
                day
            },
            episodes
            isAdult
        }
    }
}
";
