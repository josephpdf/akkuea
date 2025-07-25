'use client';

import type React from 'react';

import { useState, useEffect, useRef } from 'react';
import { Search, MessageCircle, User } from 'lucide-react';
import { Input } from '@/components/ui/input';
import Link from 'next/link';
import AkkueaLogo from '@/components/logo/akkuea-logo';
import { useMessages } from '@/store/messaging-store';
import { MessagePreview } from '@/components/messages/message-preview';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useWallet } from '@/components/auth/hooks/useWallet.hook';
import { useGlobalAuthenticationStore } from '@/components/auth/store/data';
import { Button } from '@/components/ui/button';
import { usePostsStore } from '@/store/postsStore';
import { useRouter } from 'next/navigation';
import { LogOut, Settings } from 'lucide-react';

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from '@/components/ui/dropdown-menu';

const Navbar = () => {
  const [searchQuery, setSearchQuery] = useState('');
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const searchRef = useRef<HTMLFormElement>(null);
  const { conversations } = useMessages();
  const unreadCount = conversations.reduce((count, conv) => count + (conv.unread ? 1 : 0), 0);
  const { handleConnect, handleDisconnect } = useWallet();
  const address = useGlobalAuthenticationStore((state) => state.address);
  const { searchPosts, clearSearch } = usePostsStore();
  const router = useRouter();

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (searchRef.current && !searchRef.current.contains(event.target as Node)) {
        setShowSuggestions(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  useEffect(() => {
    if (searchQuery.length > 0) {
      // simulated Suggestions
      const simulatedSuggestions = [
        `${searchQuery} en Akkuea`,
        `Buscar ${searchQuery}`,
        `${searchQuery} populares`,
      ];
      setSuggestions(simulatedSuggestions);
      setShowSuggestions(true);
    } else {
      setSuggestions([]);
      setShowSuggestions(false);
      clearSearch();
    }
  }, [searchQuery, clearSearch]);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  const handleSuggestionClick = (suggestion: string) => {
    setSearchQuery(suggestion);
    setShowSuggestions(false);
    searchPosts(suggestion);
    router.push('/');
  };

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      searchPosts(searchQuery);
      setShowSuggestions(false);
      router.push('/');
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (searchQuery.trim()) {
        searchPosts(searchQuery);
        setShowSuggestions(false);
        router.push('/');
      }
    }
  };

  return (
    <nav className="fixed top-0 right-0 left-0 border-b bg-background text-foreground z-50">
      <div className="h-14 flex items-center justify-between px-4">
        {/* Logo */}
        <Link href="/" className="flex items-center">
          <AkkueaLogo className="h-8 w-auto" />
        </Link>

        {/* Search Bar */}
        <form onSubmit={handleSearch} className="flex-1 max-w-3xl mx-4 relative" ref={searchRef}>
          <Input
            type="search"
            placeholder="Buscar posts..."
            className="w-full pl-10 h-10 bg-input border-border text-foreground placeholder:text-muted-foreground"
            value={searchQuery}
            onChange={handleSearchChange}
            onKeyDown={handleKeyPress}
          />
          <Button
            type="submit"
            size="icon"
            variant="ghost"
            className="absolute left-2 top-1/2 -translate-y-1/2 h-6 w-6 p-0"
          >
            <Search className="h-4 w-4 text-muted-foreground" />
            <span className="sr-only">Buscar</span>
          </Button>

          {/* Suggestions */}
          {showSuggestions && suggestions.length > 0 && (
            <div className="absolute z-10 w-full bg-card border border-border mt-1 rounded-md shadow-lg">
              {suggestions.map((suggestion, index) => (
                <div
                  key={index}
                  className="px-4 py-2 hover:bg-muted cursor-pointer text-foreground"
                  onClick={() => handleSuggestionClick(suggestion)}
                >
                  {suggestion}
                </div>
              ))}
            </div>
          )}
        </form>

        {/* Navigation Icons */}
        <div className="flex items-center gap-4">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Link
                  href="/messages-private"
                  className="p-2 hover:bg-muted rounded-full transition-colors relative"
                >
                  <MessageCircle className="h-5 w-5" style={{ color: '#59C9D0' }} />
                  {unreadCount > 0 && (
                    <span className="absolute -top-1 -right-1 bg-[#00CECE] text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                      {unreadCount}
                    </span>
                  )}
                </Link>
              </TooltipTrigger>
              <TooltipContent side="bottom">
                <MessagePreview />
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon" className="rounded-full">
                <User className="h-5 w-5 text-muted-foreground" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-56">
              <DropdownMenuLabel className="font-semibold text-teal-500 dark:text-teal-400">
                Jefferson Calderon
                <div className="text-xs text-muted-foreground">@xJeffx23</div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => router.push('/profile')} className="gap-2">
                <User className="h-4 w-4" /> My Profile
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => router.push('/settings')} className="gap-2">
                <Settings className="h-4 w-4" /> Settings
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={handleDisconnect} className="text-red-500 gap-2">
                <LogOut className="h-4 w-4" /> Log out
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          {address ? (
            <Button
              onClick={handleDisconnect}
              className="bg-[#59C9D0] hover:bg-[#4ab5bc] text-white font-medium px-4 py-2 rounded-full transition-colors duration-200 text-sm shadow-sm hover:shadow-md"
            >
              Disconnect
            </Button>
          ) : (
            <Button
              onClick={handleConnect}
              className="bg-[#59C9D0] hover:bg-[#4ab5bc] text-white font-medium px-4 py-2 rounded-full transition-colors duration-200 text-sm shadow-sm hover:shadow-md"
            >
              Connect
            </Button>
          )}
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
