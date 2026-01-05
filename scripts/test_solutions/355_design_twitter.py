from collections import defaultdict

class Twitter:
    def __init__(self):
        self.tweets = []
        self.follows = defaultdict(set)
        self.time = 0

    def postTweet(self, userId, tweetId):
        self.tweets.append((self.time, userId, tweetId))
        self.time += 1

    def getNewsFeed(self, userId):
        following = self.follows[userId] | {userId}
        feed = [(t, uid, tid) for t, uid, tid in self.tweets if uid in following]
        feed.sort(reverse=True)
        return [tid for _, _, tid in feed[:10]]

    def follow(self, followerId, followeeId):
        self.follows[followerId].add(followeeId)

    def unfollow(self, followerId, followeeId):
        self.follows[followerId].discard(followeeId)
