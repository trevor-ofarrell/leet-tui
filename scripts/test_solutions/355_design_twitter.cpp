class Twitter {
public:
    Twitter() : time(0) {}

    void postTweet(int userId, int tweetId) {
        tweets.push_back({time++, userId, tweetId});
    }

    vector<int> getNewsFeed(int userId) {
        unordered_set<int> following = follows[userId];
        following.insert(userId);

        vector<tuple<int, int, int>> feed;
        for (auto& t : tweets) {
            if (following.count(get<1>(t))) {
                feed.push_back(t);
            }
        }

        sort(feed.begin(), feed.end(), [](auto& a, auto& b) {
            return get<0>(a) > get<0>(b);
        });

        vector<int> result;
        for (int i = 0; i < min(10, (int)feed.size()); i++) {
            result.push_back(get<2>(feed[i]));
        }
        return result;
    }

    void follow(int followerId, int followeeId) {
        follows[followerId].insert(followeeId);
    }

    void unfollow(int followerId, int followeeId) {
        follows[followerId].erase(followeeId);
    }

private:
    vector<tuple<int, int, int>> tweets;  // (time, userId, tweetId)
    unordered_map<int, unordered_set<int>> follows;
    int time;
};
