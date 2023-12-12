// Copyright Sergey Nechoroshev 2023
#include <algorithm>
#include <climits>
#include <cstdint>
#include <exception>
#include <iterator>
#include <map>
#include <set>
#include <stack>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using std::string;
using std::vector;

class ForwardLinkedList {
 public:
  struct ListNode {
    [[maybe_unused]] int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    [[maybe_unused]] explicit ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
  };
};

// Code for solution of task 97 on leetcode
class Solution_97 {
 public:
  bool isInterleave(string s1, string s2, string s3) {
    if (s1.length() + s2.length() != s3.length()) return false;

    vector<vector<int>> dp(s1.length() + 1,
                           vector<int>(s2.length() + 1, false));
    dp[s1.length()][s2.length()] = true;
    for (int i = s1.length(); i >= 0; --i) {
      for (int j = s2.length(); j >= 0; --j) {
        if (i < s1.length() && s1[i] == s3[i + j] && dp[i + 1][j]) {
          dp[i][j] = true;
        }
        if (j < s2.length() && s2[j] == s3[i + j] && dp[i][j + 1]) {
          dp[i][j] = true;
        }
      }
    }

    return dp[0][0];
  }
};

class Solution_494 {
 private:
  int findTargetSumWaysHelper(const vector<int> &nums, int target,
                              std::map<std::pair<int, int>, int> dp, int index,
                              int total) {
    if (index >= nums.size()) {
      return (total == target) ? 1 : 0;
    }
    if (dp.contains({index, total})) {
      return dp.at({index, total});
    }
    dp[{index, total}] = findTargetSumWaysHelper(nums, target, dp, index + 1,
                                                 total + nums[index]) +
                         findTargetSumWaysHelper(nums, target, dp, index + 1,
                                                 total - nums[index]);

    return dp.at({index, total});
  }

 public:
  int findTargetSumWays(const vector<int> &nums, int target) {
    std::map<std::pair<int, int>, int> dp;
    return findTargetSumWaysHelper(nums, target, dp, 0, 0);
  }
};

class Solution_213 {
 public:
  int rob(const vector<int> &nums) {
    int n = nums.size();
    if (n == 1) return nums[0];

    int maxRobFromFirstHouse = robHelper(nums, 0, n - 2);
    int maxRobFromSecondHouse = robHelper(nums, 1, n - 1);

    return std::max(maxRobFromFirstHouse, maxRobFromSecondHouse);
  }

  int robHelper(const vector<int> &nums, int start, int end) {
    int prevRob = 0;
    int currRob = 0;

    for (int i = start; i <= end; ++i) {
      int temp = currRob;
      currRob = std::max(prevRob + nums[i], currRob);
      prevRob = temp;
    }

    return currRob;
  }
};

class Solution_198 {
 public:
  int rob(const vector<int> &nums) {
    if (nums.size() == 1) return nums[0];
    vector<int> maxRobWithHouse(nums.size());
    maxRobWithHouse[0] = nums[0];
    maxRobWithHouse[1] = nums[1];
    auto maxRob{std::max(nums[1], nums[0])};
    for (int i{2}; i < nums.size(); ++i) {
      auto maxRobBeforeThisHouse = *std::max_element(
          maxRobWithHouse.begin(), maxRobWithHouse.begin() + i - 1);
      maxRobWithHouse[i] = maxRobBeforeThisHouse + nums[i];
      maxRob = std::max(maxRob, maxRobWithHouse[i]);
    }
    return maxRob;
  }
};

class Solution_746 {
 public:
  int minCostClimbingStairs(const vector<int> &cost) {
    auto upperStair{cost.size() - 1};
    vector<int> minCostTo(cost.size(), INT_MAX);
    minCostTo[0] = cost[0];
    minCostTo[1] = cost[1];
    for (int stair{2}; stair <= upperStair; ++stair) {
      minCostTo[stair] =
          std::min(minCostTo[stair - 1], minCostTo[stair - 2]) + cost[stair];
    }
    auto result = std::min(minCostTo[upperStair], minCostTo[upperStair - 1]);
    return result;
  }
};

class Solution_70 {
 public:
  int climbStairs(int n) {
    vector<int> countOfWaysToStair(n + 1, 0);
    countOfWaysToStair[1] = 1;
    countOfWaysToStair[0] = 1;
    for (int stair{2}; stair <= n; ++stair) {
      countOfWaysToStair[stair] += countOfWaysToStair[stair - 1];
      countOfWaysToStair[stair] += countOfWaysToStair[stair - 2];
    }
    return countOfWaysToStair[n];
  }
};

class Solution_322 {
 public:
  int coinChange(const vector<int> &coins, int amount) {
    vector<int> minCountOfCoinsToGetAmount(amount + 1, INT32_MAX);
    minCountOfCoinsToGetAmount[0] = 0;
    for (int i{1}; i <= amount; ++i) {
      for (const auto &coin : coins) {
        if (i - coin >= 0) {
          minCountOfCoinsToGetAmount[i] =
              std::min(minCountOfCoinsToGetAmount[i] - 1,
                       minCountOfCoinsToGetAmount[i - coin]) +
              1;
        }
      }
    }
    auto result{(minCountOfCoinsToGetAmount[amount] == INT32_MAX)
                    ? -1
                    : minCountOfCoinsToGetAmount[amount]};
    return result;
  }
};

class Solution_647 {
 private:
  int getCountEvenPalindromes(const string &s) {
    auto counter{0};
    for (int i{1}; i < s.length(); ++i) {
      auto leftPointer{i - 1}, rightPointer{i};
      while (leftPointer >= 0 && rightPointer < s.length() &&
             leftPointer <= rightPointer) {
        if (s[leftPointer] == s[rightPointer]) {
          counter += 1;
          leftPointer -= 1;
          rightPointer += 1;
          continue;
        }
        break;
      }
    }
    return counter;
  }

  int getCountOddPalindromes(const string &s) {
    auto counter{0};
    for (int i{0}; i < s.length(); ++i) {
      auto leftPointer{i}, rightPointer{i};
      while (leftPointer >= 0 && rightPointer < s.length() &&
             leftPointer <= rightPointer) {
        if (s[leftPointer] == s[rightPointer]) {
          counter += 1;
          leftPointer -= 1;
          rightPointer += 1;
          continue;
        }
        break;
      }
    }
    return counter;
  }

 public:
  int countSubstrings(const string &s) {
    auto countOfEvenPolindromes{getCountEvenPalindromes(s)};
    auto countOfOddPolindromes{getCountOddPalindromes(s)};
    auto resultCount{countOfEvenPolindromes + countOfOddPolindromes};

    return resultCount;
  }
};

class Solution_300_binsearch {
 public:
  int lengthOfLIS(const vector<int> &nums) {
    if (nums.size() == 0) return 0;

    vector<int> curSubsequense;
    curSubsequense.emplace_back(nums[0]);

    for (auto i{1}; i < nums.size(); ++i) {
      if (curSubsequense.back() < nums[i]) {
        curSubsequense.emplace_back(nums[i]);
        continue;
      }
      auto it = std::lower_bound(curSubsequense.begin(), curSubsequense.end(),
                                 nums[i]);
      *it = nums[i];
    }
    return curSubsequense.size();
  }
};

class Solution_300 {
 public:
  int lengthOfLIS(const vector<int> &nums) {
    int maxLength{0}, numsLength{static_cast<int>(nums.size())};
    vector<int> maxLengthOfLISinPosition(numsLength, 1);
    for (int i{0}; i < numsLength; ++i) {
      for (int j{0}; j < i; ++j) {
        if (nums[j] < nums[i] &&
            maxLengthOfLISinPosition[j] + 1 > maxLengthOfLISinPosition[i]) {
          maxLengthOfLISinPosition[i] = maxLengthOfLISinPosition[j] + 1;
        }
      }
      maxLength = std::max(maxLength, maxLengthOfLISinPosition[i]);
    }
    return maxLength;
  }
};

class Solution_76 {
 public:
  string minWindow(const string &s, const string &t) {
    if (t.empty()) return "";
    std::unordered_map<char, int> countT, countWindow;
    for (const auto &ch : t) {
      auto [it, inserted] = countT.try_emplace(ch, 1);
      if (!inserted) it->second++;
    }

    int need = static_cast<int>(countT.size()), have = 0;
    int left = 0, resLen = INT32_MAX;
    std::array<int, 2> res = {-1, -1};

    for (int right = 0; right < s.length(); ++right) {
      auto curChar{s[right]};
      auto [it, inserted] = countWindow.try_emplace(curChar, 1);
      if (!inserted) it->second++;

      if (countT.contains(curChar) && countT[curChar] == countWindow[curChar]) {
        have += 1;
      }

      while (have == need) {
        if (right - left + 1 < resLen) {
          res[0] = left;
          res[1] = right;
          resLen = right - left + 1;
        }
        countWindow[s[left]]--;
        if (countT.contains(s[left]) &&
            countWindow[s[left]] < countT[s[left]]) {
          have -= 1;
        }
        left += 1;
      }
    }
    return resLen == INT32_MAX ? "" : s.substr(res[0], resLen);
  }
};
class Solution_2815 {
 private:
  int getMaxDigitFromNum(const string &num) {
    return static_cast<int>(*std::max_element(num.begin(), num.end()));
  }

 public:
  int maxSum(const vector<int> &nums) {
    std::unordered_map<int, int> maxNumOfDigit;
    int maxSum{-1};
    for (const int &num : nums) {
      auto maxDigit{getMaxDigitFromNum(std::to_string(num))};
      if (maxNumOfDigit.contains(maxDigit)) {
        maxSum = std::max(maxSum, num + maxNumOfDigit[maxDigit]);
        maxNumOfDigit[maxDigit] = std::max(maxNumOfDigit[maxDigit], num);
        continue;
      }
      maxNumOfDigit.try_emplace(maxDigit, num);
    }
    return maxSum;
  }
};
class Solution_4 {
 public:
  double findMedianSortedArrays(vector<int> nums1, vector<int> nums2) {
    if (nums1.size() > nums2.size()) {
      std::swap(nums1, nums2);
    }
    int totalSize{static_cast<int>(nums1.size() + nums2.size())};
    int half{totalSize / 2}, left{0}, right{static_cast<int>(nums1.size())};

    while (true) {
      auto mid{left + (right - left) / 2};
      auto secondHalf{half - mid - 2};

      auto nums1Left{(mid >= 0) ? nums1[mid] : INT_MIN};
      auto nums1Right{(mid + 1 < nums1.size()) ? nums1[mid + 1] : INT_MAX};

      auto nums2Left{(mid >= 0) ? nums2[secondHalf] : INT_MIN};
      auto nums2Right{(mid + 1 < nums2.size()) ? nums2[secondHalf + 1]
                                               : INT_MAX};

      if (nums1Left <= nums2Right && nums2Left <= nums1Right) {
        return (totalSize % 2 == 0)
                   ? static_cast<double>(std::max(nums1Left, nums2Left) +
                                         std::min(nums1Right, nums2Right)) /
                         2.0
                   : std::min(nums1Left, nums2Right);
      } else if (nums1Left > nums2Right) {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }
  }
};

class Solution_567 {
 private:
  bool isPermuntation(const std::unordered_map<char, int> &countOfLetter) {
    for (const auto &[letter, count] : countOfLetter) {
      if (count != 0) return false;
    }
    return true;
  }

 public:
  bool checkInclusion(string s1, string s2) {
    auto s1Len{s1.length()}, s2Len(s2.length());
    if (s1Len > s2Len) return false;
    std::unordered_map<char, int> countOfLetter;
    for (auto i = 'a'; i <= 'z'; ++i) {
      countOfLetter.try_emplace(i, 0);
    }
    for (int i = 0; i < s1Len; ++i) {
      countOfLetter[s1[i]]++;
      countOfLetter[s1[i]]--;
    }
    if (isPermuntation(countOfLetter)) return true;
    for (auto i = s1Len; i < s2Len; ++i) {
      countOfLetter[s2[i]]--;
      countOfLetter[s2[i - s1Len]]++;
      if (isPermuntation(countOfLetter)) return true;
    }
    return false;
  }
};

class Solution_424 {
 public:
  int characterReplacement(string s, int k) {
    std::unordered_map<char, int> countLettersInCurrentWindow;
    int countMostRepeatedLetter{0}, left{0}, maxLength{0};
    for (int right{0}; right < s.length(); right++) {
      auto curLen{right - left + 1};
      countMostRepeatedLetter = std::max(
          countMostRepeatedLetter, ++countLettersInCurrentWindow[s[right]]);
      if (curLen - countMostRepeatedLetter > k) {
        countLettersInCurrentWindow[s[left]]--;
        left++;
        curLen--;
      }

      maxLength = std::max(maxLength, curLen);
    }
    return maxLength;
  }
};
class Solution_206 {
  using ListNode = ForwardLinkedList::ListNode;

 public:
  ListNode *reverseList(ListNode *head) {
    auto previousNode{head};
    auto currentNode{head->next};
    while (currentNode) {
      auto NextNode{currentNode->next};
      currentNode->next = previousNode;
      currentNode = NextNode;
    }
    return previousNode;
  }
};

class Solution_3 {
 public:
  int lengthOfLongestSubstring(string s) {
    std::set<char> currentSubstring;
    int left{0}, maxLength{0};
    for (const auto &rightValue : s) {
      while (currentSubstring.contains(rightValue)) {
        currentSubstring.erase(s[left]);
        left++;
      }
      currentSubstring.insert(rightValue);
      maxLength =
          std::max(maxLength, static_cast<int>(currentSubstring.size()));
    }
    return maxLength;
  }
};

class Solution_121 {
 public:
  int maxProfit(const vector<int> &prices) {
    int sizePrices{static_cast<int>(prices.size())};
    if (sizePrices == 1 || sizePrices == 0) {
      return 0;
    }
    int left{0}, right{1}, maxProfit{0};
    while (left <= right && right < sizePrices) {
      auto curProfit{prices[right] - prices[left]};
      maxProfit = std::max(maxProfit, curProfit);
      if (prices[right] >= prices[left]) {
        right++;
      } else {
        left = right;
      }
    }
    return maxProfit;
  }
};

class Solution_84 {
 public:
  int largestRectangleArea(const vector<int> &heights) {
    // Sergey TODO: Comprehend this solution
    std::stack<std::pair<int, int>> stackIndexiesAndHeights;
    int maxArea{0};
    for (auto i{0}; i < heights.size(); ++i) {
      auto start{i};
      while (!stackIndexiesAndHeights.empty() &&
             stackIndexiesAndHeights.top().second > heights[i]) {
        auto curIndex{stackIndexiesAndHeights.top().first};
        auto width{i - curIndex};
        auto height{stackIndexiesAndHeights.top().second};
        stackIndexiesAndHeights.pop();

        maxArea = std::max(maxArea, width * height);
        start = curIndex;
      }
      stackIndexiesAndHeights.push(std::make_pair(start, heights[i]));
    }

    while (!stackIndexiesAndHeights.empty()) {
      auto width{static_cast<int>(heights.size()) -
                 stackIndexiesAndHeights.top().first};
      auto height{stackIndexiesAndHeights.top().second};
      stackIndexiesAndHeights.pop();

      maxArea = std::max(maxArea, width * height);
    }
    return maxArea;
  }
};

class Solution_42_refactored {
 public:
  int trap(const vector<int> &height) {
    int leftPointer{0}, rightPointer{static_cast<int>(height.size()) - 1};
    int maxLeft{height[leftPointer]}, maxRight{height[rightPointer]};
    int amountWater{0};
    while (leftPointer < rightPointer) {
      if (maxLeft < maxRight) {
        leftPointer++;
        maxLeft = std::max(maxLeft, height[leftPointer]);
        amountWater += maxLeft - height[leftPointer];
      } else {
        rightPointer--;
        maxRight = std::max(maxRight, height[rightPointer]);
        amountWater += maxRight - height[rightPointer];
      }
    }
    return amountWater;
  }
};

class Solution_42 {
 public:
  int trap(const vector<int> &height) {
    int countBlocks{static_cast<int>(height.size())};
    vector<int> maxLeft(countBlocks), maxRight(countBlocks);
    maxLeft[0] = 0;
    maxRight[countBlocks - 1] = 0;
    for (int i{1}; i < countBlocks; ++i) {
      maxLeft[i] = std::max(height[i - 1], maxLeft[i - 1]);
      maxRight[countBlocks - i - 1] =
          std::max(height[countBlocks - i], maxRight[countBlocks - i]);
    }

    int countBlocksOfWater{0};
    for (int i{0}; i < countBlocks; ++i) {
      auto currentAmountWater = std::min(maxLeft[i], maxRight[i]) - height[i];
      if (currentAmountWater > 0) {
        countBlocksOfWater += currentAmountWater;
      }
    }
    return countBlocksOfWater;
  }
};

class Solution_33 {
 public:
  int search(const vector<int> &nums, int target) {
    int left{0}, right{static_cast<int>(nums.size()) - 1}, mid;
    while (left <= right) {
      mid = left + (right - left) / 2;

      if (nums[mid] == target) {
        return mid;
      }

      if (nums[mid] >= nums[left]) {
        if (target >= nums[left] && target < nums[mid]) {
          right = mid - 1;
        } else {
          left = mid + 1;
        }
      } else {
        if (target > nums[mid] && target <= nums[right]) {
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      }
    }
    return -1;
  }
};

class Solution_153 {
 public:
  int findMin(const vector<int> &nums) {
    int left{0}, right{static_cast<int>(nums.size()) - 1}, mid;
    while (left < right) {
      mid = left + (right - left) / 2;
      if (nums[mid] > nums[right]) {
        left = mid + 1;
      } else {
        right = mid;
      }
    }
    return nums[mid];
  }
};

class Solution_875 {
 private:
  bool isSpeedEnough(const vector<int> &piles, int h, int speedOfEating) {
    if (!speedOfEating) return false;
    int64_t hoursOfEating{0};
    for (const auto &pile : piles) {
      hoursOfEating += std::ceil(pile / static_cast<double>(speedOfEating));
    }
    return hoursOfEating <= h;
  }

 public:
  int minEatingSpeed(const vector<int> &piles, int h) {
    int64_t minEatingSpeed{0}, maxEatingSpeed{INT_MAX + 1l}, midEatingSpeed;
    while (minEatingSpeed < maxEatingSpeed) {
      midEatingSpeed = minEatingSpeed + (maxEatingSpeed - minEatingSpeed) / 2;
      auto isThisSpeedEnough = isSpeedEnough(piles, h, midEatingSpeed);
      if (isThisSpeedEnough) {
        maxEatingSpeed = midEatingSpeed;
      } else {
        minEatingSpeed = midEatingSpeed + 1;
      }
    }
    return minEatingSpeed;
  }
};

class Solution_853 {
 private:
  double getTimeToTarget(int target, int position, int speed) {
    return std::abs(target - position) / static_cast<double>(speed);
  }

 public:
  int carFleet(int target, const vector<int> &position,
               const vector<int> &speed) {
    vector<std::pair<int, int>> cars(position.size());
    for (int i{0}; i < position.size(); ++i) {
      cars[i] = std::make_pair(position[i], speed[i]);
    }
    std::sort(cars.begin(), cars.end(),
              [](auto a, auto b) { return a.first > b.first; });
    std::stack<int> fleetsInIndexes;
    fleetsInIndexes.push(0);
    for (int i{1}; i < cars.size(); ++i) {
      auto previousFleet = cars[fleetsInIndexes.top()];
      auto previousCarTimeToTarget =
          getTimeToTarget(target, previousFleet.first, previousFleet.second);
      auto currentCar = cars[i];
      auto currentCarTimeToTarget =
          getTimeToTarget(target, currentCar.first, currentCar.second);
      if (currentCarTimeToTarget > previousCarTimeToTarget) {
        fleetsInIndexes.push(i);
      }
    }
    return fleetsInIndexes.size();
  }
};

class Solution_74 {
 public:
  bool searchMatrix(const vector<vector<int>> &matrix, int target) {
    // To do with complexity O(log(n * m)) we will treat the matrix as if it
    // were a flat array
    int rows{static_cast<int>(matrix.size())},
        cols{static_cast<int>(matrix[0].size())}, left{0},
        right{rows * cols - 1}, mid, midValue;
    while (left <= right) {
      mid = left + (right - left) / 2;
      midValue = matrix[mid / cols][mid % cols];

      if (midValue == target) {
        return true;
      } else if (midValue < target) {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return false;
  }
};

class Solution_704 {
 public:
  int search(const vector<int> &nums, int target) {
    int first{0}, last{static_cast<int>(nums.size()) - 1}, mid;
    while (first <= last) {
      mid = first + (last - first) / 2;
      if (nums[mid] == target) return mid;
      if (nums[mid] < target) {
        first = mid + 1;
      } else {
        last = mid - 1;
      }
    }
    return -1;
  }
};

class Solution_11 {
 public:
  int maxArea(const vector<int> &height) {
    int left{0}, right{static_cast<int>(height.size()) - 1};
    int maxArea{0};
    while (left < right) {
      auto leftHeight{height[left]};
      auto rightHeight{height[right]};
      maxArea =
          std::max(maxArea, std::min(leftHeight, rightHeight) * (right - left));
      if (leftHeight > rightHeight) {
        right--;
      } else {
        left++;
      }
    }
    return maxArea;
  }
};

class Solution_15 {
 public:
  vector<vector<int>> threeSum(vector<int> nums) {
    vector<vector<int>> result;
    std::sort(nums.begin(), nums.end());
    for (size_t i{0}; i < nums.size(); ++i) {
      if (i > 0 && nums[i] == nums[i - 1]) continue;
      size_t left{i + 1}, right{nums.size() - 1};
      while (left < right) {
        auto sum = nums[i] + nums[left] + nums[right];
        if (sum == 0) {
          result.push_back({nums[i], nums[left], nums[right]});
          while (left < right && nums[left] == nums[left + 1]) left++;
          while (left < right && nums[right] == nums[right - 1]) right--;
          left++;
          right--;
          continue;
        }
        if (sum < 0) {
          left++;
        } else {
          right--;
        }
      }
    }
    return result;
  }
};

class Solution_167 {
 public:
  vector<int> twoSum(const vector<int> &numbers, int target) {
    size_t firstPointer{0}, lastPointer{numbers.size() - 1};
    while (firstPointer < lastPointer) {
      auto sum = numbers[firstPointer] + numbers[lastPointer];
      if (sum == target) break;
      if (sum > target) {
        lastPointer--;
      } else {
        firstPointer++;
      }
    }
    vector<int> result{static_cast<int>(firstPointer) + 1,
                       static_cast<int>(lastPointer) + 1};
    return result;
  }
};

class Solution_125 {
 public:
  bool isPalindrome(string s) {
    if (s.empty()) {
      return true;
    }

    int firstPointer = 0;
    int lastPointer = s.length() - 1;

    while (firstPointer < lastPointer) {
      if (!std::isalnum(s[firstPointer])) {
        ++firstPointer;
      } else if (!std::isalnum(s[lastPointer])) {
        --lastPointer;
      } else {
        if (std::tolower(s[firstPointer]) != std::tolower(s[lastPointer])) {
          return false;
        }
        ++firstPointer;
        --lastPointer;
      }
    }

    return true;
  }
};

class Solution_739 {
 public:
  vector<int> dailyTemperatures(const vector<int> &temperatures) {
    std::stack<int> waitingStack;
    vector<int> daysToWarmer(temperatures.size(), 0);

    for (int i = 0; i < temperatures.size(); ++i) {
      while (!waitingStack.empty() &&
             temperatures[i] > temperatures[waitingStack.top()]) {
        int prevIndex = waitingStack.top();
        waitingStack.pop();
        daysToWarmer[prevIndex] = i - prevIndex;
      }
      waitingStack.push(i);
    }
    return daysToWarmer;
  }
};

class Solution_22 {
 private:
  void generateParenthesis(int openN, int closedN, int n, string current,
                           vector<string> *result) {
    if (openN == closedN && closedN == n) {
      result->push_back(current);
      return;
    }

    if (openN < n) {
      generateParenthesis(openN + 1, closedN, n, current + "(", result);
    }

    if (closedN < openN) {
      generateParenthesis(openN, closedN + 1, n, current + ")", result);
    }
  }

 public:
  vector<string> generateParenthesis(int n) {
    vector<string> result;
    generateParenthesis(0, 0, n, "", &result);
    return result;
  }
};

class Solution_150 {
 public:
  /**
   * Evaluates the given reverse polish notation (RPN) expression and returns
   * the result.
   *
   * @param tokens A vector of strings representing the RPN expression.
   *
   * @return The result of evaluating the RPN expression.
   *
   * @throws std::invalid_argument If there is a division by zero or if no
   * result is found.
   */
  int evalRPN(const vector<string> &tokens) {
    std::unordered_map<string, int (*)(int, int)> operations{
        {"+", [](int a, int b) { return a + b; }},
        {"-", [](int a, int b) { return a - b; }},
        {"*", [](int a, int b) { return a * b; }},
        {"/",
         [](int a, int b) {
           if (b == 0) throw std::invalid_argument("Division by zero");
           return a / b;
         }},
    };
    std::stack<int> operands;

    for (const auto &token : tokens) {
      if (operations.contains(token)) {
        auto right_operand = operands.top();
        operands.pop();
        auto left_operand = operands.top();
        operands.pop();
        operands.push(operations.at(token)(left_operand, right_operand));
        continue;
      }
      operands.push(std::stoi(token));
    }
    if (operands.empty()) {
      throw std::invalid_argument("No result found");
    }
    return operands.top();
  }
};

class Solution_155 {
 public:
  class MinStack {
   public:
    MinStack() {}

    void push(int val) {
      mainStack.push(val);
      if (minStack.empty() || val <= minStack.top()) {
        minStack.push(val);
      }
    }

    void pop() {
      if (mainStack.empty()) {
        throw std::out_of_range("Stack is empty");
      }
      if (mainStack.top() == minStack.top()) {
        minStack.pop();
      }
      mainStack.pop();
    }

    int top() {
      if (mainStack.empty()) {
        throw std::out_of_range("Stack is empty");
      }
      return mainStack.top();
    }

    int getMin() {
      if (mainStack.empty()) {
        throw std::out_of_range("Stack is empty");
      }
      return minStack.top();
    }

   private:
    std::stack<int> mainStack;
    std::stack<int> minStack;
  };
};

class Solution_20 {
 public:
  /**
   * Check if a string containing brackets is valid.
   *
   * @param s the string containing brackets
   *
   * @return true if the string is valid, false otherwise
   *
   * @throws None
   */
  bool isValid(const string &s) {
    std::stack<char> stack_of_brackets;
    std::unordered_map<char, char> brackets{{')', '('}, {']', '['}, {'}', '{'}};

    for (const auto &ch : s) {
      if (ch == '(' || ch == '[' || ch == '{') {
        stack_of_brackets.push(ch);
        continue;
      }
      if (stack_of_brackets.empty() ||
          brackets[ch] != stack_of_brackets.top()) {
        return false;
      }
      stack_of_brackets.pop();
    }
    return stack_of_brackets.empty();
  }
};

class Solution_128_refactored {
 public:
  /**
   * Finds the length of the longest consecutive subsequence in a given vector
   * of integers.
   *
   * @param nums the vector of integers to search for the longest consecutive
   * subsequence
   *
   * @return the length of the longest consecutive subsequence
   *
   * @throws None
   */
  int longestConsecutive(const vector<int> &nums) {
    // But this version slower, but I think it's smarter
    std::unordered_set<int> set_numbers(nums.begin(), nums.end());
    int longest_seq_len{0};
    for (const auto &number : set_numbers) {
      if (set_numbers.contains(number - 1)) {
        continue;
      }
      int length{1};
      while (set_numbers.contains(number + length)) {
        length += 1;
      }
      longest_seq_len = std::max(length, longest_seq_len);
    }
    return longest_seq_len;
  }
};
class Solution_128 {
 public:
  /**
   * Finds the length of the longest consecutive sequence in a vector of
   * integers.
   *
   * @param nums a vector of integers
   *
   * @return the length of the longest consecutive sequence
   *
   * @throws None
   */
  int longestConsecutive(vector<int> nums) {
    if (nums.size() == 0) return 0;
    std::sort(nums.begin(), nums.end());
    int longest_consecutive_len = 1;
    int current_consecutive_len = 1;
    for (int i{1}; i < nums.size(); ++i) {
      if (nums[i] == nums[i - 1]) {
        continue;
      }
      if (nums[i] - nums[i - 1] == 1) {
        current_consecutive_len += 1;
        continue;
      }
      longest_consecutive_len =
          std::max(current_consecutive_len, longest_consecutive_len);
      current_consecutive_len = 1;
    }
    return std::max(current_consecutive_len, longest_consecutive_len);
  }
};
class Solution_659 {
 public:
  /*
   * @param strs: a list of strings
   * @return: encodes a list of strings to a single string.
   */
  string encode(const vector<string> &strs) {
    string encoded_str = "";
    for (const auto &str : strs) {
      encoded_str += str + "|_|";
    }
    return encoded_str;
  }

  /*
   * @param str: A string
   * @return: decodes a single string to a list of strings
   */
  vector<string> decode(const string &str) {
    vector<string> decoded_strs;
    auto current_iter = str.begin();
    auto end_iter = str.end();
    auto start_search_position_iter = current_iter;
    while (current_iter != end_iter) {
      auto start_code_sequense_iter =
          std::find(start_search_position_iter, end_iter, '|');
      if (*(start_code_sequense_iter + 1) == '_' &&
          *(start_code_sequense_iter + 2) == '|') {
        decoded_strs.push_back(string(current_iter, start_code_sequense_iter));
        current_iter = start_code_sequense_iter + 3;
      }
      start_search_position_iter =
          std::max(current_iter, start_code_sequense_iter + 1);
    }
    return decoded_strs;
  }
};

class Solution_36_refactored {
 public:
  bool isValidSudoku(const vector<vector<char>> &board) {
    vector<vector<std::unordered_set<char>>> seen_num_in_subboxes(
        3, vector<std::unordered_set<char>>(3));
    vector<std::unordered_set<char>> nums_in_rows(board.size());
    vector<std::unordered_set<char>> nums_in_columns(board.size());
    for (int i{0}; i < board.size(); ++i) {
      for (int j{0}; j < board.size(); ++j) {
        if (board[i][j] == '.') continue;
        bool is_seen_in_row = nums_in_rows[i].contains(board[i][j]);
        bool is_seen_in_column = nums_in_columns[j].contains(board[i][j]);
        bool is_seen_in_subboxe =
            seen_num_in_subboxes[i / 3][j / 3].contains(board[i][j]);
        if (is_seen_in_row || is_seen_in_column || is_seen_in_subboxe) {
          return false;
        }
        nums_in_rows[i].insert(board[i][j]);
        nums_in_columns[i].insert(board[i][j]);
        seen_num_in_subboxes[i / 3][j / 3].insert(board[i][j]);
      }
    }
    return true;
  }
};
class Solution_36 {
 private:
  bool is_row_sudoku_valid(const vector<vector<char>> &board) {
    for (const auto &row : board) {
      bool is_row_valid = check_nums_sequense(row);
      if (!is_row_valid) {
        return false;
      }
    }
    return true;
  }

  bool check_nums_sequense(const vector<char> &nums) {
    std::unordered_set<char> viewed_nums;
    for (const char &num : nums) {
      if (num == '.') {
        continue;
      }
      bool is_num_valid = '0' <= num && num <= '9';
      if (!is_num_valid) {
        return false;
      }
      bool is_num_sequense_non_valid = viewed_nums.contains(num);
      if (is_num_sequense_non_valid) {
        return false;
      }
      viewed_nums.insert(num);
    }
    return true;
  }

  bool is_boxe_valid(int index_row, int index_coloumn,
                     const vector<vector<char>> &board) {
    try {
      vector<std::pair<int, int>> nums_directions = {{-1, 0}, {-1, 1},  {0, 1},
                                                     {1, 1},  {1, 0},   {1, -1},
                                                     {0, -1}, {-1, -1}, {0, 0}};
      vector<char> current_nums;
      for (const auto &direction : nums_directions) {
        current_nums.push_back(board[index_row + direction.first]
                                    [index_coloumn + direction.second]);
      }
      auto is_sequense_valid = check_nums_sequense(current_nums);
      return is_sequense_valid;
    } catch (std::exception) {
      return false;
    }
    return false;
  }
  bool is_subboxes_sudoku_valid(const vector<vector<char>> &board) {
    int board_size = board.size();
    for (int i{1}; i < board_size; i += 3) {
      for (int j{1}; j < board_size; j += 3) {
        auto is_subboxe_valid = is_boxe_valid(i, j, board);
        if (!is_subboxe_valid) {
          return false;
        }
      }
    }
    return true;
  }

  bool is_column_sudoku(const vector<vector<char>> &board) {
    auto board_size = board.size();
    for (int i{0}; i < board_size; ++i) {
      vector<char> column;
      for (int j{0}; j < board_size; ++j) {
        column.push_back(board[j][i]);
      }
      bool is_sequense_valid = check_nums_sequense(column);
      if (!is_sequense_valid) return false;
    }
    return true;
  }

 public:
  bool isValidSudoku(const vector<vector<char>> &board) {
    bool is_rows_valid = is_row_sudoku_valid(board);
    if (!is_rows_valid) return false;

    bool _is_colums_valid = is_column_sudoku(board);
    if (!_is_colums_valid) {
      return false;
    }

    bool is_subboxes_valid = is_subboxes_sudoku_valid(board);
    if (!is_subboxes_valid) {
      return false;
    }
    return true;
  }
};

class Solution_238 {
 public:
  vector<int> productExceptSelf(const vector<int> &nums) {
    int nums_size = nums.size();
    vector<int> product_prefixes(nums_size, 1);
    vector<int> product_postfixes(nums_size, 1);
    for (int i{1}; i < nums_size; ++i) {
      product_prefixes[i] = nums[i - 1] * product_prefixes[i - 1];
      product_postfixes[nums_size - i - 1] =
          nums[nums_size - i] * product_postfixes[nums_size - i];
    }

    vector<int> result(nums_size);
    result[0] = product_postfixes[0];
    result[nums_size - 1] = product_prefixes[nums_size - 1];
    for (int i{1}; i < nums_size - 1; ++i) {
      result[i] = product_prefixes[i] * product_postfixes[i];
    }
    return result;
  }
};

class Solution_347 {
 public:
  vector<int> topKFrequent(const vector<int> &nums, int k) {
    std::unordered_map<int, int> number_of_nums;
    for (const auto &num : nums) {
      number_of_nums[num]++;
    }
    vector<std::pair<int, int>> number_of_nums_v;
    for (const auto &[num, number] : number_of_nums) {
      number_of_nums_v.push_back({num, number});
    }
    std::sort(number_of_nums_v.begin(), number_of_nums_v.end(),
              [&](std::pair<int, int> a, std::pair<int, int> b) {
                return a.second > b.second;
              });
    vector<int> result(k);
    for (int i{0}; i < k; ++i) {
      result[i] = number_of_nums_v[i].first;
    }
    return result;
  }
};

class Solution_49 {
 public:
  vector<vector<string>> groupAnagrams(const vector<string> &strs) {
    std::unordered_map<string, vector<string>> groups_of_anagrams;
    for (const auto &word : strs) {
      auto sorted_word = word;
      std::sort(sorted_word.begin(), sorted_word.end());
      groups_of_anagrams[sorted_word].push_back(word);
    }
    vector<vector<string>> result;
    for (const auto &[key, group] : groups_of_anagrams) {
      result.push_back(group);
    }
    return result;
  }
};

class Solution_1 {
 public:
  vector<int> twoSum(const vector<int> &nums, int target) {
    std::unordered_map<int, int> index_for_needed_num;
    for (auto i{0}; i < nums.size(); ++i) {
      auto needed_num = target - nums[i];
      if (index_for_needed_num.contains(needed_num)) {
        return {index_for_needed_num[needed_num], i};
      }
      index_for_needed_num[nums[i]] = i;
    }
    return {-1, -1};
  }
};

class Solution_242 {
 public:
  bool isAnagram(string s, string t) {
    if (s.length() != t.length()) return false;

    auto number_letter = s.length();
    std::unordered_multiset<char> multiset_s;
    std::unordered_multiset<char> multiset_t;
    for (auto i{0}; i < number_letter; ++i) {
      multiset_s.insert(s[i]);
      multiset_t.insert(t[i]);
    }

    return multiset_s == multiset_t;
  }
};

class Solution_217 {
 public:
  bool containsDuplicate(vector<int> nums) {
    std::unordered_set<int> encountered_elements;
    for (const auto &num : nums) {
      if (encountered_elements.contains(num)) {
        return true;
      }
      encountered_elements.insert(num);
    }
    return false;
  }
};

class Solution_1431 {
 public:
  std::vector<bool> kidsWithCandies(const vector<int> &candies,
                                    int extraCandies) {
    auto candies_size = candies.size();
    vector<bool> result(candies_size);
    auto biggest_number_of_candies =
        *std::max_element(candies.begin(), candies.end());
    for (auto i{0}; i < candies_size; ++i) {
      result[i] = (candies[i] + extraCandies) >= biggest_number_of_candies;
    }
    return result;
  }
};

class Solution_1512_refactored {
 public:
  int numIdenticalPairs(const vector<int> &nums) {
    std::unordered_map<int, int> count;
    int count_of_good_pairs{0};
    for (const auto &num : nums) {
      count_of_good_pairs += count[num]++;
    }
    return count_of_good_pairs;
  }
};

class Solution_1512 {
 public:
  int numIdenticalPairs(const vector<int> &nums) {
    int count_of_good_pairs{0};
    for (auto i{0}; i < nums.size(); ++i) {
      for (auto j{i + 1}; j < nums.size(); ++j) {
        if (nums[i] == nums[j]) {
          ++count_of_good_pairs;
        }
      }
    }
    return count_of_good_pairs;
  }
};

class Solution_1929 {
 public:
  vector<int> getConcatenation(const std::vector<int> &nums) {
    vector<int> result;
    for (auto i{0}; i < 2; ++i) {
      std::copy(begin(nums), end(nums), back_inserter(result));
    }
    return result;
  }
};
