pub const ANILIST_ICON: &str = "https://media.discordapp.net/attachments/1101850602050424832/1101850610959130655/icon.png?width=344&height=344";
pub const ANILIST_ANIME_QUERY: &str = "
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
pub const ANILIST_MANGA_QUERY: &str = "
query($search: String, $page: Int = 1, $per_page: Int = 10) {
    Page(page: $page, perPage: $per_page) {
        pageInfo {
            total
            currentPage
            lastPage
        }
        media(search: $search, type: MANGA, sort: POPULARITY_DESC) {
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
            volumes
            isAdult
        }
    }
}
";