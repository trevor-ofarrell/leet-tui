var Twitter = function() { this.tweets = []; this.follows = new Map(); this.time = 0; };
Twitter.prototype.postTweet = function(userId, tweetId) { this.tweets.push({ userId, tweetId, time: this.time++ }); };
Twitter.prototype.getNewsFeed = function(userId) {
    const following = this.follows.get(userId) || new Set();
    following.add(userId);
    return this.tweets.filter(t => following.has(t.userId)).sort((a, b) => b.time - a.time).slice(0, 10).map(t => t.tweetId);
};
Twitter.prototype.follow = function(followerId, followeeId) {
    if (!this.follows.has(followerId)) this.follows.set(followerId, new Set());
    this.follows.get(followerId).add(followeeId);
};
Twitter.prototype.unfollow = function(followerId, followeeId) {
    if (this.follows.has(followerId)) this.follows.get(followerId).delete(followeeId);
};
